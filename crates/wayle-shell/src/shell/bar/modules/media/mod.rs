mod helpers;
mod messages;

use std::sync::Arc;

use gtk::prelude::WidgetExt;
use relm4::prelude::*;
use tokio_util::sync::CancellationToken;
use tracing::error;
use wayle_common::{
    ConfigProperty, WatcherToken, process::spawn_shell_quiet, services, watch, watch_cancellable,
};
use wayle_config::{
    ConfigService,
    schemas::{
        modules::{MediaConfig, MediaIconType},
        styling::CssToken,
    },
};
use wayle_media::{MediaService, core::player::Player, types::PlaybackState};
use wayle_widgets::prelude::{
    BarButton, BarButtonBehavior, BarButtonColors, BarButtonInit, BarButtonInput, BarButtonOutput,
};

use self::helpers::{FormatContext, IconContext, format_label, resolve_icon};
pub(crate) use self::messages::{MediaCmd, MediaInit, MediaMsg};

pub(crate) struct MediaModule {
    bar_button: Controller<BarButton>,
    visible: ConfigProperty<bool>,
    player_watcher: WatcherToken,
}

#[relm4::component(pub(crate))]
impl Component for MediaModule {
    type Init = MediaInit;
    type Input = MediaMsg;
    type Output = ();
    type CommandOutput = MediaCmd;

    view! {
        gtk::Box {
            #[local_ref]
            bar_button -> gtk::MenuButton {},
        }
    }

    fn init(
        init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let config_service = services::get::<ConfigService>();
        let config = config_service.config();
        let media_config = &config.modules.media;

        let visible = ConfigProperty::new(false);

        let bar_button = BarButton::builder()
            .launch(BarButtonInit {
                icon: media_config.icon_name.get().clone(),
                label: String::from("No media"),
                tooltip: None,
                colors: BarButtonColors {
                    icon_color: media_config.icon_color.clone(),
                    label_color: media_config.label_color.clone(),
                    icon_background: media_config.icon_bg_color.clone(),
                    button_background: media_config.button_bg_color.clone(),
                    border_color: media_config.border_color.clone(),
                    auto_icon_color: CssToken::Blue,
                },
                behavior: BarButtonBehavior {
                    label_max_chars: media_config.label_max_length.clone(),
                    show_icon: media_config.icon_show.clone(),
                    show_label: media_config.label_show.clone(),
                    show_border: media_config.border_show.clone(),
                    visible: visible.clone(),
                },
                settings: init.settings,
            })
            .forward(sender.input_sender(), |output| match output {
                BarButtonOutput::LeftClick => MediaMsg::LeftClick,
                BarButtonOutput::RightClick => MediaMsg::RightClick,
                BarButtonOutput::MiddleClick => MediaMsg::MiddleClick,
                BarButtonOutput::ScrollUp => MediaMsg::ScrollUp,
                BarButtonOutput::ScrollDown => MediaMsg::ScrollDown,
            });

        Self::spawn_watchers(&sender, media_config);

        let model = Self {
            bar_button,
            visible,
            player_watcher: WatcherToken::new(),
        };
        let bar_button = model.bar_button.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        let config_service = services::get::<ConfigService>();
        let media_config = &config_service.config().modules.media;

        let cmd = match msg {
            MediaMsg::LeftClick => media_config.left_click.get().clone(),
            MediaMsg::RightClick => media_config.right_click.get().clone(),
            MediaMsg::MiddleClick => media_config.middle_click.get().clone(),
            MediaMsg::ScrollUp => media_config.scroll_up.get().clone(),
            MediaMsg::ScrollDown => media_config.scroll_down.get().clone(),
        };

        if !cmd.is_empty()
            && let Err(e) = spawn_shell_quiet(&cmd)
        {
            error!(error = %e, cmd = %cmd, "failed to spawn command");
        }
    }

    fn update_cmd(&mut self, msg: MediaCmd, sender: ComponentSender<Self>, root: &Self::Root) {
        let config_service = services::get::<ConfigService>();
        let media_config = &config_service.config().modules.media;

        match msg {
            MediaCmd::PlayerChanged(player) => {
                self.visible.set(player.is_some());

                let use_disc =
                    player.is_some() && media_config.icon_type.get() == MediaIconType::SpinningDisc;
                Self::update_disc_mode(root, use_disc);

                if let Some(player) = player {
                    let label = Self::build_label(media_config, &player);
                    self.bar_button.emit(BarButtonInput::SetLabel(label));

                    let icon = Self::build_icon(media_config, &player);
                    self.bar_button.emit(BarButtonInput::SetIcon(icon));

                    let state = player.playback_state.get();
                    Self::update_spinning_state(root, state);

                    let token = self.player_watcher.reset();
                    Self::spawn_player_watchers(&sender, &player, token);
                }
            }
            MediaCmd::MetadataChanged => {
                let media_service = services::get::<MediaService>();
                if let Some(player) = media_service.active_player() {
                    let label = Self::build_label(media_config, &player);
                    self.bar_button.emit(BarButtonInput::SetLabel(label));
                }
            }
            MediaCmd::PlaybackStateChanged => {
                let media_service = services::get::<MediaService>();
                if let Some(player) = media_service.active_player() {
                    let label = Self::build_label(media_config, &player);
                    self.bar_button.emit(BarButtonInput::SetLabel(label));
                    let state = player.playback_state.get();
                    Self::update_spinning_state(root, state);
                }
            }
            MediaCmd::UpdateIcon(icon) => {
                self.bar_button.emit(BarButtonInput::SetIcon(icon));
            }
            MediaCmd::IconTypeChanged => {
                let media_service = services::get::<MediaService>();
                let use_disc = media_service.active_player().is_some()
                    && media_config.icon_type.get() == MediaIconType::SpinningDisc;
                Self::update_disc_mode(root, use_disc);

                if let Some(player) = media_service.active_player() {
                    let icon = Self::build_icon(media_config, &player);
                    self.bar_button.emit(BarButtonInput::SetIcon(icon));

                    let state = player.playback_state.get();
                    Self::update_spinning_state(root, state);
                }
            }
        }
    }
}

impl MediaModule {
    fn spawn_watchers(sender: &ComponentSender<Self>, config: &MediaConfig) {
        let media_service = services::get::<MediaService>();

        let active_stream = media_service.active_player.watch();
        watch!(sender, [active_stream], |out| {
            let media_service = services::get::<MediaService>();
            let _ = out.send(MediaCmd::PlayerChanged(media_service.active_player()));
        });

        let format = config.format.clone();
        watch!(sender, [format.watch()], |out| {
            let _ = out.send(MediaCmd::MetadataChanged);
        });

        let icon_name = config.icon_name.clone();
        let icon_type = config.icon_type.clone();
        watch!(sender, [icon_name.watch()], |out| {
            if icon_type.get() == MediaIconType::Default {
                let _ = out.send(MediaCmd::UpdateIcon(icon_name.get().clone()));
            }
        });

        let spinning_disc_icon = config.spinning_disc_icon.clone();
        let icon_type_for_disc = config.icon_type.clone();
        watch!(sender, [spinning_disc_icon.watch()], |out| {
            if icon_type_for_disc.get() == MediaIconType::SpinningDisc {
                let _ = out.send(MediaCmd::UpdateIcon(spinning_disc_icon.get().clone()));
            }
        });

        watch!(sender, [config.icon_type.watch()], |out| {
            let _ = out.send(MediaCmd::IconTypeChanged);
        });
    }

    fn spawn_player_watchers(
        sender: &ComponentSender<Self>,
        player: &Arc<Player>,
        token: CancellationToken,
    ) {
        let metadata = player.metadata.clone();
        let metadata_token = token.clone();
        watch_cancellable!(sender, metadata_token, [metadata.watch()], |out| {
            let _ = out.send(MediaCmd::MetadataChanged);
        });

        let playback_state = player.playback_state.clone();
        watch_cancellable!(sender, token, [playback_state.watch()], |out| {
            let _ = out.send(MediaCmd::PlaybackStateChanged);
        });
    }

    fn build_label(config: &MediaConfig, player: &Player) -> String {
        let format = config.format.get();
        let title = player.metadata.title.get();
        let artist = player.metadata.artist.get();
        let album = player.metadata.album.get();
        format_label(&FormatContext {
            format: &format,
            title: &title,
            artist: &artist,
            album: &album,
            state: player.playback_state.get(),
        })
    }

    fn build_icon(config: &MediaConfig, player: &Player) -> String {
        let icon_name = config.icon_name.get();
        let spinning_disc_icon = config.spinning_disc_icon.get();
        let player_icons = config.player_icons.get();
        resolve_icon(&IconContext {
            icon_type: config.icon_type.get(),
            icon_name: &icon_name,
            spinning_disc_icon: &spinning_disc_icon,
            player_icons: &player_icons,
            bus_name: player.id.bus_name(),
            desktop_entry: player.desktop_entry.get().as_deref(),
        })
    }

    fn update_disc_mode(root: &gtk::Box, enabled: bool) {
        if enabled {
            root.add_css_class("media-disc");
        } else {
            root.remove_css_class("media-disc");
        }
    }

    fn update_spinning_state(root: &gtk::Box, state: PlaybackState) {
        match state {
            PlaybackState::Playing => {
                root.add_css_class("media-spinning");
            }
            PlaybackState::Paused | PlaybackState::Stopped => {
                root.remove_css_class("media-spinning");
            }
        }
    }
}
