//! `hook-quiet`: observes every lifecycle event and changes nothing. A wasm32-wasip2
//! component exporting `murmur:hook/lifecycle` (world `hook`).

#[cfg(target_arch = "wasm32")]
mod wasm_hook {
    wit_bindgen::generate!({
        path: "../../wit/hook",
        world: "hook",
        generate_all,
    });

    use exports::murmur::hook::lifecycle::{
        CompactionEvent, Guest, HookOutput, InferenceEvent, SessionContext, SessionEndEvent,
        ShellEvent, StageEvent, TaskEndEvent, TaskStartEvent, ToolEvent,
    };

    struct QuietHook;

    impl Guest for QuietHook {
        fn on_stage(_e: StageEvent) -> Result<HookOutput, String> { Ok(HookOutput::None) }
        fn on_session_start(_e: SessionContext) -> Result<HookOutput, String> { Ok(HookOutput::None) }
        fn on_task_start(_e: TaskStartEvent) -> Result<HookOutput, String> { Ok(HookOutput::None) }
        fn on_inference(_e: InferenceEvent) -> Result<HookOutput, String> { Ok(HookOutput::None) }
        fn on_tool_call(_e: ToolEvent) -> Result<HookOutput, String> { Ok(HookOutput::None) }
        fn on_shell(_e: ShellEvent) -> Result<HookOutput, String> { Ok(HookOutput::None) }
        fn on_compaction(_e: CompactionEvent) -> Result<HookOutput, String> { Ok(HookOutput::None) }
        fn on_task_end(_e: TaskEndEvent) -> Result<HookOutput, String> { Ok(HookOutput::None) }
        fn on_session_end(_e: SessionEndEvent) -> Result<HookOutput, String> { Ok(HookOutput::None) }
    }

    export!(QuietHook);
}
