#[cfg(target_os = "macos")]
mod mac;

#[cfg(target_os = "windows")]
mod win;

#[cfg(target_os = "linux")]
mod linux;

#[derive(Debug, Clone)]
pub struct Window {
    pub id: u32,
    pub title: String,

    #[cfg(target_os = "windows")]
    pub raw_handle: windows::Win32::Foundation::HWND,

    #[cfg(target_os = "macos")]
    pub raw_handle: core_graphics_helmer_fork::window::CGWindowID,
}

#[derive(Debug, Clone)]
pub struct Display {
    pub id: u32,
    pub title: String,

    #[cfg(target_os = "windows")]
    pub raw_handle: windows::Win32::Graphics::Gdi::HMONITOR,

    #[cfg(target_os = "macos")]
    pub raw_handle: core_graphics_helmer_fork::display::CGDisplay,
}

impl PartialEq<Self> for Display {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
unsafe impl Send for Display{}
impl Eq for Display{

}


impl ToString for Display{
    fn to_string(&self) -> String {
        self.title.clone() + " " +
            &get_target_dimensions(&Target::Display(self.clone())).0.to_string() + "x" +
            &get_target_dimensions(&Target::Display(self.clone())).1.to_string()
    }
}

#[derive(Debug, Clone)]
pub enum Target {
    Window(Window),
    Display(Display),
}

/// Returns a list of targets that can be captured
pub fn get_all_targets() -> Vec<Target> {
    #[cfg(target_os = "macos")]
    return mac::get_all_targets();

    #[cfg(target_os = "windows")]
    return win::get_all_targets();

    #[cfg(target_os = "linux")]
    return linux::get_all_targets();
}

pub fn get_scale_factor(target: &Target) -> f64 {
    #[cfg(target_os = "macos")]
    return mac::get_scale_factor(target);

    #[cfg(target_os = "windows")]
    return win::get_scale_factor(target);

    #[cfg(target_os = "linux")]
    return 1.0;
}

pub fn get_main_display() -> Display {
    #[cfg(target_os = "macos")]
    return mac::get_main_display();

    #[cfg(target_os = "windows")]
    return win::get_main_display();

    #[cfg(target_os = "linux")]
    unreachable!();
}

pub fn get_target_dimensions(target: &Target) -> (u64, u64) {
    #[cfg(target_os = "macos")]
    return mac::get_target_dimensions(target);

    #[cfg(target_os = "windows")]
    return win::get_target_dimensions(target);

    #[cfg(target_os = "linux")]
    unreachable!();
}

pub fn get_all_displays() -> Vec<Display> {
    // Esegui immediatamente scappare::get_all_targets() e filtra solo i Display
    get_all_targets() // Chiama la funzione esistente per ottenere i target
        .into_iter() // Crea un iteratore su tutti i target
        .filter_map(|target| {
            // Applica il filtro, accettando solo i target di tipo Display
            if let Target::Display(display) = target {
                Some(display) // Ritorna il Display se trovato
            } else {
                None // Ignora tutti gli altri tipi
            }
        })
        .collect() // Raccogli i risultati filtrati in un Vec<Display>
}
