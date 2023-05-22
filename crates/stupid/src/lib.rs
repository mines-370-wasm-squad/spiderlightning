use slight_common::impl_resource;

wit_bindgen_wasmtime::export!("../../wit/stupid.wit");

#[derive(Debug, Clone, Default)]
pub struct Stupid;

impl_resource!(
    Stupid,
    stupid::StupidTables<Stupid>,
    stupid::add_to_linker,
    "stupid".to_string()
);

impl stupid::Stupid for Stupid {
    type Stupid = Stupid;

    fn stupid_open_browser(&mut self, url: &str) -> Result<(), String> {
        webbrowser::open(url).map_err(|e| e.to_string())
    }
}
