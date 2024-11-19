import { invoke } from "@tauri-apps/api/core";

export const downloadExecutableFile = async (id=1) => {
    return await invoke<string>("download_executable_file", {id,});
}