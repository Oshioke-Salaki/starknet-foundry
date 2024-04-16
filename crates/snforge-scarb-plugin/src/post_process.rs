use cairo_lang_macro::PostProcessContext;

pub const FULL_PATH_TEST_MARKER: &str = "test_marker";

pub fn _post_process(_context: PostProcessContext) {
    // let paths = context
    //     .full_path_markers
    //     .into_iter()
    //     .filter_map(|marker| (marker.key == FULL_PATH_TEST_MARKER).then(|| marker.full_path));

    // let metadata = ScarbCommand::metadata().run().unwrap();

    // let profile = &metadata.current_profile;
    // let target_name = &metadata.compilation_units[0].target.name;

    // let sierra_file = File::open(format!("./target/{profile}/{target_name}.sierra.json")).unwrap();

    // let sierra: VersionedProgram = serde_json::from_reader(sierra_file).unwrap();
}
