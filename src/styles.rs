//! Constantes de estilo do window manager.
//! Cores no formato 0xRRGGBBAA (o penrose 0.4 espera o alfa no final).

pub const PROFONT: &str = "JetBrainsMono Nerd Font";

pub mod colors {
    pub const BLACK: u32 = 0x000000ff;
    pub const GREY: u32 = 0x808080ff;
    pub const WHITE: u32 = 0xffffffff;
    pub const PURPLE: u32 = 0xa020f0ff;

    // Ainda não usadas no main.rs.
    #[allow(dead_code)]
    pub const BLUE: u32 = 0x0000ffff;
    #[allow(dead_code)]
    pub const RED: u32 = 0xff0000ff;
}

pub mod dimensions {
    /// Altura da barra em pixels. Também é o espaço reservado no topo pelos layouts.
    pub const HEIGHT: u32 = 18;
    /// Tamanho da fonte da barra, em pontos.
    pub const FONT_SIZE: u8 = 11;
}
