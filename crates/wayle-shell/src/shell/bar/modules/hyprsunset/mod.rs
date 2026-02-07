mod factory;
mod helpers;
mod messages;
mod watchers;

use std::sync::Arc;

use gtk::prelude::*;
use relm4::prelude::*;
use tracing::debug;
use wayle_common::{ConfigProperty, process};
use wayle_config::{ConfigService, schemas::styling::CssToken};
use wayle_widgets::prelude::{
    BarButton, BarButtonBehavior, BarButtonColors, BarButtonInit, BarButtonInput, BarButtonOutput,
};

use self::helpers::LabelContext;
pub(crate) use self::{
    factory::Factory,
    messages::{HyprsunsetCmd, HyprsunsetInit, HyprsunsetMsg},
};

pub(crate) struct HyprsunsetModule {
    bar_button: Controller<BarButton>,
    config: Arc<ConfigService>,
    enabled: bool,
    current_temp: u32,
    current_gamma: u32,
}

#[relm4::component(pub(crate))]
impl Component for HyprsunsetModule {
    type Init = HyprsunsetInit;
    type Input = HyprsunsetMsg;
    type Output = ();
    type CommandOutput = HyprsunsetCmd;

    view! {
        gtk::Box {
            add_css_class: "hyprsunset",

            #[local_ref]
            bar_button -> gtk::MenuButton {},
        }
    }

    fn init(
        init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let config_service = init.config;
        let config = config_service.config().modules.hyprsunset.clone();

        let bar_button = BarButton::builder()
            .launch(BarButtonInit {
                icon: config.icon_off.get().clone(),
                label: String::new(),
                tooltip: None,
                colors: BarButtonColors {
                    icon_color: config.icon_color.clone(),
                    label_color: config.label_color.clone(),
                    icon_background: config.icon_bg_color.clone(),
                    button_background: config.button_bg_color.clone(),
                    border_color: config.border_color.clone(),
                    auto_icon_color: CssToken::Yellow,
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
                BarButtonOutput::LeftClick => HyprsunsetMsg::LeftClick,
                BarButtonOutput::RightClick => HyprsunsetMsg::RightClick,
                BarButtonOutput::MiddleClick => HyprsunsetMsg::MiddleClick,
                BarButtonOutput::ScrollUp => HyprsunsetMsg::ScrollUp,
                BarButtonOutput::ScrollDown => HyprsunsetMsg::ScrollDown,
            });

        watchers::spawn_config_watchers(&sender, &config);
        watchers::spawn_state_watcher(&sender);

        let model = Self {
            bar_button,
            config: config_service,
            enabled: false,
            current_temp: config.temperature.get(),
            current_gamma: config.gamma.get(),
        };
        let bar_button = model.bar_button.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        let config = &self.config.config().modules.hyprsunset;

        match msg {
            HyprsunsetMsg::LeftClick => {
                let action = config.left_click.get();
                if action == ":toggle" {
                    self.toggle_filter(&sender, config);
                } else {
                    process::run_if_set(&action);
                }
            }
            HyprsunsetMsg::RightClick => process::run_if_set(&config.right_click.get()),
            HyprsunsetMsg::MiddleClick => process::run_if_set(&config.middle_click.get()),
            HyprsunsetMsg::ScrollUp => process::run_if_set(&config.scroll_up.get()),
            HyprsunsetMsg::ScrollDown => process::run_if_set(&config.scroll_down.get()),
        }
    }

    fn update_cmd(
        &mut self,
        msg: HyprsunsetCmd,
        _sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        let config = &self.config.config().modules.hyprsunset;

        match msg {
            HyprsunsetCmd::ConfigChanged => {
                self.update_display(config);
            }
            HyprsunsetCmd::StateChanged(state) => {
                let enabled = state.is_some();
                let (temp, gamma) = state
                    .map(|s| (s.temp, s.gamma))
                    .unwrap_or((config.temperature.get(), config.gamma.get()));

                if self.enabled != enabled
                    || self.current_temp != temp
                    || self.current_gamma != gamma
                {
                    debug!(enabled, temp, gamma, "hyprsunset state changed");
                    self.enabled = enabled;
                    self.current_temp = temp;
                    self.current_gamma = gamma;
                    self.update_display(config);
                }
            }
        }
    }
}

impl HyprsunsetModule {
    fn toggle_filter(
        &self,
        sender: &ComponentSender<Self>,
        config: &wayle_config::schemas::modules::HyprsunsetConfig,
    ) {
        let enabled = self.enabled;
        let temp = config.temperature.get();
        let gamma = config.gamma.get();

        debug!(current_enabled = enabled, "toggle_filter called");

        sender.oneshot_command(async move {
            if enabled {
                debug!("stopping hyprsunset");
                let _ = helpers::stop().await;
                HyprsunsetCmd::StateChanged(None)
            } else {
                debug!(temp, gamma, "starting hyprsunset");
                let _ = helpers::start(temp, gamma).await;
                HyprsunsetCmd::StateChanged(Some(helpers::HyprsunsetState { temp, gamma }))
            }
        });
    }

    fn update_display(&self, config: &wayle_config::schemas::modules::HyprsunsetConfig) {
        let icon =
            helpers::select_icon(self.enabled, &config.icon_off.get(), &config.icon_on.get());
        self.bar_button.emit(BarButtonInput::SetIcon(icon));

        let label = helpers::build_label(&LabelContext {
            format: &config.format.get(),
            temp: self.current_temp,
            gamma: self.current_gamma,
            config_temp: config.temperature.get(),
            config_gamma: config.gamma.get(),
            enabled: self.enabled,
        });
        self.bar_button.emit(BarButtonInput::SetLabel(label));
    }
}
