mod helpers;
mod messages;
mod watchers;

use relm4::{gtk::prelude::*, prelude::*};
use tracing::warn;
use wayle_common::{ConfigProperty, process, services};
use wayle_config::{ConfigService, schemas::styling::CssToken};
use wayle_idle_inhibit::IdleInhibitor;
use wayle_widgets::prelude::{
    BarButton, BarButtonBehavior, BarButtonColors, BarButtonInit, BarButtonInput, BarButtonOutput,
};

use self::helpers::LabelContext;
pub(crate) use self::messages::{IdleInhibitCmd, IdleInhibitInit, IdleInhibitMsg};
use crate::services::idle_inhibit::{IdleInhibitService, IdleInhibitState};

pub(crate) struct IdleInhibitModule {
    bar_button: Controller<BarButton>,
    state: IdleInhibitState,
    inhibitor: Option<IdleInhibitor>,
}

#[relm4::component(pub(crate))]
impl Component for IdleInhibitModule {
    type Init = IdleInhibitInit;
    type Input = IdleInhibitMsg;
    type Output = ();
    type CommandOutput = IdleInhibitCmd;

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
        let config = &config_service.config().modules.idle_inhibit;

        let idle_service = services::get::<IdleInhibitService>();
        let state = idle_service.state();

        let bar_button = BarButton::builder()
            .launch(BarButtonInit {
                icon: config.icon_inactive.get().clone(),
                label: String::new(),
                tooltip: None,
                colors: BarButtonColors {
                    icon_color: config.icon_color.clone(),
                    label_color: config.label_color.clone(),
                    icon_background: config.icon_bg_color.clone(),
                    button_background: config.button_bg_color.clone(),
                    border_color: config.border_color.clone(),
                    auto_icon_color: CssToken::Green,
                },
                behavior: BarButtonBehavior {
                    label_max_chars: config.label_max_length.clone(),
                    show_icon: config.icon_show.clone(),
                    show_label: config.label_show.clone(),
                    show_border: config.border_show.clone(),
                    visible: ConfigProperty::new(true),
                },
                settings: init.settings,
            })
            .forward(sender.input_sender(), |output| match output {
                BarButtonOutput::LeftClick => IdleInhibitMsg::LeftClick,
                BarButtonOutput::RightClick => IdleInhibitMsg::RightClick,
                BarButtonOutput::MiddleClick => IdleInhibitMsg::MiddleClick,
                BarButtonOutput::ScrollUp => IdleInhibitMsg::ScrollUp,
                BarButtonOutput::ScrollDown => IdleInhibitMsg::ScrollDown,
            });

        watchers::spawn_config_watchers(&sender, config);
        watchers::spawn_state_watchers(&sender, &state);

        let model = Self {
            bar_button,
            state,
            inhibitor: None,
        };
        let bar_button = model.bar_button.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        let config_service = services::get::<ConfigService>();
        let config = &config_service.config().modules.idle_inhibit;

        match msg {
            IdleInhibitMsg::LeftClick => process::run_if_set(&config.left_click.get()),
            IdleInhibitMsg::RightClick => process::run_if_set(&config.right_click.get()),
            IdleInhibitMsg::MiddleClick => process::run_if_set(&config.middle_click.get()),
            IdleInhibitMsg::ScrollUp => process::run_if_set(&config.scroll_up.get()),
            IdleInhibitMsg::ScrollDown => process::run_if_set(&config.scroll_down.get()),
        }
    }

    fn update_cmd(
        &mut self,
        msg: IdleInhibitCmd,
        _sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        let config_service = services::get::<ConfigService>();
        let config = &config_service.config().modules.idle_inhibit;

        match msg {
            IdleInhibitCmd::ConfigChanged | IdleInhibitCmd::StateChanged => {
                self.sync_inhibitor();
                self.update_display(config);
            }
        }
    }
}

impl IdleInhibitModule {
    fn sync_inhibitor(&mut self) {
        let should_be_active = self.state.active.get();
        let is_active = self.inhibitor.is_some();

        if should_be_active && !is_active {
            self.create_inhibitor();
        } else if !should_be_active && is_active {
            self.inhibitor.take();
        }
    }

    fn create_inhibitor(&mut self) {
        let widget = self.bar_button.widget();
        let Some(native) = widget.native() else {
            warn!("widget has no native surface");
            return;
        };
        let Some(gdk_surface) = native.surface() else {
            warn!("native has no surface");
            return;
        };
        let Some(inhibitor) = IdleInhibitor::new(&gdk_surface) else {
            warn!("failed to create idle inhibitor");
            return;
        };

        self.inhibitor = Some(inhibitor);
    }

    fn update_display(&self, config: &wayle_config::schemas::modules::IdleInhibitConfig) {
        let active = self.state.active.get();

        let icon = helpers::select_icon(
            active,
            &config.icon_inactive.get(),
            &config.icon_active.get(),
        );
        self.bar_button.emit(BarButtonInput::SetIcon(icon));

        let label = helpers::build_label(
            &config.label_format.get(),
            &LabelContext {
                active,
                duration_mins: self.state.duration_mins.get(),
                remaining_secs: self.state.remaining_secs.get(),
            },
        );
        self.bar_button.emit(BarButtonInput::SetLabel(label));
    }
}

impl Drop for IdleInhibitModule {
    fn drop(&mut self) {
        self.inhibitor.take();
    }
}
