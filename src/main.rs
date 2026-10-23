mod hooks;
mod styles;

use std::collections::HashMap;

use penrose::{
    builtin::{
        actions::{exit, modify_with, send_layout_message, spawn},
        layout::{
            messages::{ExpandMain, IncMain, ShrinkMain},
            transformers::ReserveTop,
            MainAndStack, Monocle,
        },
    },
    core::{
        bindings::{parse_keybindings_with_xmodmap, KeyEventHandler},
        hooks::StateHook,
        layout::LayoutStack,
        Config, WindowManager,
    },
    extensions::hooks::ewmh::add_ewmh_hooks,
    map, stack,
    x11rb::RustConn,
};
use penrose_ui::{status_bar, Position, TextStyle};
use simplelog::{LevelFilter, SimpleLogger};

use styles::{colors, dimensions, PROFONT};

// Aplicativos padrão e scripts
pub const TERMINAL: &str = "kitty";
pub const LAUNCHER: &str = "dmenu_run";
pub const PATH_TO_START_SCRIPT: &str = "~/.config/mywm/scripts/start.sh";

// Workspaces e layouts
const TAGS: [&str; 5] = ["1", "2", "3", "4", "5"];
const MAX_MAIN: u32 = 1;
const RATIO: f32 = 0.6;
const RATIO_STEP: f32 = 0.1;

fn layouts() -> LayoutStack {
    stack!(
        MainAndStack::side(MAX_MAIN, RATIO, RATIO_STEP),
        MainAndStack::bottom(MAX_MAIN, RATIO, RATIO_STEP),
        Box::new(Monocle)
    )
    // Reserva o topo da tela para a barra de status.
    .map(|layout| ReserveTop::wrap(layout, dimensions::HEIGHT))
}

fn raw_key_bindings() -> HashMap<String, Box<dyn KeyEventHandler<RustConn>>> {
    let mut raw_bindings = map! {
        map_keys: |k: &str| k.to_string();
        
        
        // Volume
        "XF86AudioRaiseVolume" => spawn("wpctl set-volume -l 1.0 @DEFAULT_AUDIO_SINK@ 5%+"),
        "XF86AudioLowerVolume" => spawn("wpctl set-volume -l 1.0 @DEFAULT_AUDIO_SINK@ 5%-"),
        "XF86AudioMute" => spawn("wpctl set-mute @DEFAULT_AUDIO_SINK@ toggle"),

        // Brilho
        "XF86MonBrightnessUp" => spawn("brightnessctl set 5%+"),
        "XF86MonBrightnessDown" => spawn("brightnessctl set 5%-"),
	
        // Programas
        "M-p" => spawn(LAUNCHER),
        "M-Return" => spawn(TERMINAL),

        // Sair do penrose (importante lembrar desta!)
        "M-A-C-Escape" => exit(),

        // Clientes
        "M-j" => modify_with(|cs| cs.focus_down()),
        "M-k" => modify_with(|cs| cs.focus_up()),
        "M-S-j" => modify_with(|cs| cs.swap_down()),
        "M-S-k" => modify_with(|cs| cs.swap_up()),
        "M-c" => modify_with(|cs| cs.kill_focused()),

        // Workspaces
        "M-Tab" => modify_with(|cs| cs.toggle_tag()),

        // Layouts
        "M-grave" => modify_with(|cs| cs.next_layout()),
        "M-S-grave" => modify_with(|cs| cs.previous_layout()),
        "M-A-Up" => send_layout_message(|| IncMain(1)),
        "M-A-Down" => send_layout_message(|| IncMain(-1)),
        "M-l" => send_layout_message(|| ExpandMain),
        "M-h" => send_layout_message(|| ShrinkMain),
    };

    for &tag in &TAGS {
        raw_bindings.extend([
            (
                format!("M-{tag}"),
                modify_with(move |cs| cs.focus_tag(tag)),
            ),
            (
                format!("M-S-{tag}"),
                modify_with(move |cs| cs.move_focused_to_tag(tag)),
            ),
        ]);
    }

    raw_bindings
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Logging
    if let Err(e) = SimpleLogger::init(LevelFilter::Info, simplelog::Config::default()) {
        panic!("unable to set log level: {e}");
    }

    // Config
    let config = add_ewmh_hooks(Config {
        default_layouts: layouts(),
        tags: TAGS.iter().map(|t| t.to_string()).collect(),
        normal_border: colors::GREY.into(),
        focused_border: colors::PURPLE.into(),
        startup_hook: Some(hooks::StartupScript::new(PATH_TO_START_SCRIPT).boxed()),
        ..Config::default()
    });

    // Conexão X e atalhos (parse_keybindings_with_xmodmap exige o `xmodmap` instalado)
    let conn = RustConn::new()?;
    let key_bindings = parse_keybindings_with_xmodmap(raw_key_bindings())?;
    let wm = WindowManager::new(config, key_bindings, HashMap::new(), conn)?;

    // Barra de status
    let style = TextStyle {
        fg: colors::WHITE.into(),
        bg: Some(colors::BLACK.into()),
        padding: (2, 2),
    };
    let bar = status_bar(
        dimensions::HEIGHT,
        PROFONT,
        dimensions::FONT_SIZE,
        style,
        colors::PURPLE, // destaque do workspace ativo
        colors::GREY,   // workspaces vazios
        Position::Top,
    )?;
    let wm = bar.add_to(wm);

    wm.run()?;
    Ok(())
}
