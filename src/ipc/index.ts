import { ValueBase } from "./messages/ValueBase";
import { FromJson, ToJson } from "./messages/encoders/json";
import { normValueType } from "./messages/valuetype";
import * as ms from "./messages/types";
import { invoke } from "@tauri-apps/api";
import { t } from "./messages/names";

export async function loggedErr<IN>(promise: Promise<IN>): Promise<IN> {
    return promise.catch((err: string) => {
        console.log(`ipc err: ${err}`);
        return Promise.reject(err);
    });
}

async function ipcRequest<OUT, IN>(pipe: string, message: OUT, outType: any, inType: any): Promise<IN> {
    let ldmValue = ValueBase._valueToLdm(message, normValueType(outType));
    const jsonData = ToJson.fromLdm(ldmValue);
    return loggedErr(invoke(pipe, { message: jsonData }).then((data) => {
        let encoder = new FromJson<IN>(data as string, normValueType(inType));
        return encoder.toValue();
    }));
}

export async function info(): Promise<ms.structs.AboutApp> {
    return ipcRequest("info", new ms.structs.None(), t.None, t.AboutApp);
}

export async function startScan(path: string): Promise<ms.structs.None> {
    return ipcRequest("start_scan", path, t.str, t.None);
}

export async function scanStep(timeBudgetMs: number): Promise<ms.enums.ScanState> {
    return ipcRequest("scan_step", timeBudgetMs, t.int, t.ScanState);
}

export async function getAggregateData(upToFraction: number): Promise<ms.structs.AggregateData> {
    return ipcRequest("get_aggregate_data", upToFraction, t.float, t.AggregateData);
}

export async function openPath(path: String): Promise<ms.structs.None> {
    return ipcRequest("open_path", path, t.str, t.None);
}

export async function rescan(): Promise<ms.structs.None> {
    return ipcRequest("rescan", new ms.structs.None(), t.None, t.None);
}

export async function jump(entryIndex: number): Promise<ms.structs.None> {
    return ipcRequest("jump", entryIndex, t.int, t.None);
}

export async function levelUp(): Promise<string | null> {
    return ipcRequest("level_up", new ms.structs.None(), t.None, [t.Option, t.str]);
}

export async function navigate(globalId: number | null, path: string): Promise<ms.structs.None> {
    return ipcRequest("navigate", new ms.structs.Navigation(globalId, path), t.Navigation, t.None);
}