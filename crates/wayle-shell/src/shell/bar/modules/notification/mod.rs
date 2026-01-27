mod helpers;
mod messages;
mod watchers;

use relm4::prelude::*;
use wayle_common::{ConfigProperty, process, services};
use wayle_config::{
    ConfigService,
    schemas::{modules::NotificationConfig, styling::CssToken},
};
use wayle_notification::NotificationService;
use wayle_widgets::prelude::{
    BarButton, BarButtonBehavior, BarButtonColors, BarButtonInit, BarButtonInput, BarButtonOutput,
};

use self::helpers::{IconContext, format_label, select_icon};
pub(crate) use self::messages::{NotificationCmd, NotificationInit, NotificationMsg};

pub(crate) struct NotificationModule {
    bar_button: Controller<BarButton>,
    count: usize,
    dnd: bool,
}

#[relm4::component(pub(crate))]
impl Component for NotificationModule {
    type Init = NotificationInit;
    type Input = NotificationMsg;
    type Output = ();
    type CommandOutput = NotificationCmd;

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
        let notification_config = &config.modules.notification;

        let notification_service = services::get::<NotificationService>();
        let initial_count = notification_service.notifications.get().len();
        let initial_dnd = notification_service.dnd.get();

        let initial_icon = select_icon(&IconContext {
            count: initial_count,
            dnd: initial_dnd,
            icon_name: &notification_config.icon_name.get(),
            icon_unread: &notification_config.icon_unread.get(),
            icon_dnd: &notification_config.icon_dnd.get(),
        });

        let initial_label = format_label(initial_count, notification_config.hide_empty.get());

        let bar_button = BarButton::builder()
            .launch(BarButtonInit {
                icon: initial_icon,
                label: initial_label,
                tooltip: None,
                colors: BarButtonColors {
                    icon_color: notification_config.icon_color.clone(),
                    label_color: notification_config.label_color.clone(),
                    icon_background: notification_config.icon_bg_color.clone(),
                    button_background: notification_config.button_bg_color.clone(),
                    border_color: notification_config.border_color.clone(),
                    auto_icon_color: CssToken::Green,
                },
                behavior: BarButtonBehavior {
                    label_max_chars: notification_config.label_max_length.clone(),
                    show_icon: notification_config.icon_show.clone(),
                    show_label: notification_config.label_show.clone(),
                    show_border: notification_config.border_show.clone(),
                    visible: ConfigProperty::new(true),
                },
                settings: init.settings,
            })
            .forward(sender.input_sender(), |output| match output {
                BarButtonOutput::LeftClick => NotificationMsg::LeftClick,
                BarButtonOutput::RightClick => NotificationMsg::RightClick,
                BarButtonOutput::MiddleClick => NotificationMsg::MiddleClick,
                BarButtonOutput::ScrollUp => NotificationMsg::ScrollUp,
                BarButtonOutput::ScrollDown => NotificationMsg::ScrollDown,
            });

        watchers::spawn_watchers(&sender, notification_config);

        let model = Self {
            bar_button,
            count: initial_count,
            dnd: initial_dnd,
        };
        let bar_button = model.bar_button.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        let config_service = services::get::<ConfigService>();
        let config = &config_service.config().modules.notification;

        match msg {
            NotificationMsg::LeftClick => {}
            NotificationMsg::RightClick => process::run_if_set(&config.right_click.get()),
            NotificationMsg::MiddleClick => process::run_if_set(&config.middle_click.get()),
            NotificationMsg::ScrollUp => process::run_if_set(&config.scroll_up.get()),
            NotificationMsg::ScrollDown => process::run_if_set(&config.scroll_down.get()),
        }
    }

    fn update_cmd(
        &mut self,
        msg: NotificationCmd,
        _sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        let config_service = services::get::<ConfigService>();
        let notification_config = &config_service.config().modules.notification;

        match msg {
            NotificationCmd::NotificationsChanged(count) => {
                self.count = count;
                self.update_display(notification_config);
            }
            NotificationCmd::DndChanged(dnd) => {
                self.dnd = dnd;
                self.update_display(notification_config);
            }
            NotificationCmd::IconConfigChanged => {
                self.update_display(notification_config);
            }
        }
    }
}

impl NotificationModule {
    fn update_display(&self, config: &NotificationConfig) {
        let icon_name = config.icon_name.get();
        let icon_unread = config.icon_unread.get();
        let icon_dnd = config.icon_dnd.get();

        let icon = select_icon(&IconContext {
            count: self.count,
            dnd: self.dnd,
            icon_name: &icon_name,
            icon_unread: &icon_unread,
            icon_dnd: &icon_dnd,
        });
        self.bar_button.emit(BarButtonInput::SetIcon(icon));

        let label = format_label(self.count, config.hide_empty.get());
        self.bar_button.emit(BarButtonInput::SetLabel(label));
    }
}
