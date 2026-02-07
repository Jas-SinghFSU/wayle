mod factory;
mod helpers;
mod messages;
mod watchers;

use std::sync::Arc;

use gtk::prelude::*;
use relm4::prelude::*;
use wayle_bluetooth::BluetoothService;
use wayle_common::{ConfigProperty, WatcherToken, process};
use wayle_config::{
    ConfigService,
    schemas::{modules::BluetoothConfig, styling::CssToken},
};
use wayle_widgets::prelude::{
    BarButton, BarButtonBehavior, BarButtonColors, BarButtonInit, BarButtonInput, BarButtonOutput,
};

use self::helpers::{BluetoothContext, format_label, select_icon};
pub(crate) use self::{
    factory::Factory,
    messages::{BluetoothCmd, BluetoothInit, BluetoothMsg},
};

pub(crate) struct BluetoothModule {
    bar_button: Controller<BarButton>,
    adapter_watcher: WatcherToken,
    bluetooth: Arc<BluetoothService>,
    config: Arc<ConfigService>,
}

#[relm4::component(pub(crate))]
impl Component for BluetoothModule {
    type Init = BluetoothInit;
    type Input = BluetoothMsg;
    type Output = ();
    type CommandOutput = BluetoothCmd;

    view! {
        gtk::Box {
            add_css_class: "bluetooth",

            #[local_ref]
            bar_button -> gtk::MenuButton {},
        }
    }

    fn init(
        init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let config = init.config.config();
        let bt_config = &config.modules.bluetooth;

        let (initial_icon, initial_label) = Self::compute_display(bt_config, &init.bluetooth);

        let bar_button = BarButton::builder()
            .launch(BarButtonInit {
                icon: initial_icon,
                label: initial_label,
                tooltip: None,
                colors: BarButtonColors {
                    icon_color: bt_config.icon_color.clone(),
                    label_color: bt_config.label_color.clone(),
                    icon_background: bt_config.icon_bg_color.clone(),
                    button_background: bt_config.button_bg_color.clone(),
                    border_color: bt_config.border_color.clone(),
                    auto_icon_color: CssToken::Blue,
                },
                behavior: BarButtonBehavior {
                    label_max_chars: bt_config.label_max_length.clone(),
                    show_icon: bt_config.icon_show.clone(),
                    show_label: bt_config.label_show.clone(),
                    show_border: bt_config.border_show.clone(),
                    visible: ConfigProperty::new(true),
                },
                settings: init.settings,
            })
            .forward(sender.input_sender(), |output| match output {
                BarButtonOutput::LeftClick => BluetoothMsg::LeftClick,
                BarButtonOutput::RightClick => BluetoothMsg::RightClick,
                BarButtonOutput::MiddleClick => BluetoothMsg::MiddleClick,
                BarButtonOutput::ScrollUp => BluetoothMsg::ScrollUp,
                BarButtonOutput::ScrollDown => BluetoothMsg::ScrollDown,
            });

        watchers::spawn_watchers(&sender, bt_config, &init.bluetooth);

        let mut adapter_watcher = WatcherToken::new();
        watchers::spawn_adapter_watchers(&sender, adapter_watcher.reset(), &init.bluetooth);

        let model = Self {
            bar_button,
            adapter_watcher,
            bluetooth: init.bluetooth,
            config: init.config,
        };
        let bar_button = model.bar_button.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        let config = &self.config.config().modules.bluetooth;

        let cmd = match msg {
            BluetoothMsg::LeftClick => config.left_click.get(),
            BluetoothMsg::RightClick => config.right_click.get(),
            BluetoothMsg::MiddleClick => config.middle_click.get(),
            BluetoothMsg::ScrollUp => config.scroll_up.get(),
            BluetoothMsg::ScrollDown => config.scroll_down.get(),
        };

        process::run_if_set(&cmd);
    }

    fn update_cmd(&mut self, msg: BluetoothCmd, sender: ComponentSender<Self>, _root: &Self::Root) {
        let bt_config = &self.config.config().modules.bluetooth;

        match msg {
            BluetoothCmd::StateChanged | BluetoothCmd::IconConfigChanged => {
                let (icon, label) = Self::compute_display(bt_config, &self.bluetooth);
                self.bar_button.emit(BarButtonInput::SetIcon(icon));
                self.bar_button.emit(BarButtonInput::SetLabel(label));
            }
            BluetoothCmd::AdapterChanged => {
                let token = self.adapter_watcher.reset();
                watchers::spawn_adapter_watchers(&sender, token, &self.bluetooth);

                let (icon, label) = Self::compute_display(bt_config, &self.bluetooth);
                self.bar_button.emit(BarButtonInput::SetIcon(icon));
                self.bar_button.emit(BarButtonInput::SetLabel(label));
            }
        }
    }
}

impl BluetoothModule {
    fn compute_display(config: &BluetoothConfig, bt: &BluetoothService) -> (String, String) {
        let available = bt.available.get();
        let enabled = bt.enabled.get();
        let devices = bt.devices.get();
        let connected_addresses = bt.connected.get();

        let discovering = bt
            .primary_adapter
            .get()
            .map(|a| a.discovering.get())
            .unwrap_or(false);

        let connected_devices: Vec<_> = devices
            .iter()
            .filter(|d| connected_addresses.contains(&d.address.get()))
            .cloned()
            .collect();

        let ctx = BluetoothContext {
            available,
            enabled,
            discovering,
            connected_devices: &connected_devices,
        };

        (select_icon(config, &ctx), format_label(&ctx))
    }
}
