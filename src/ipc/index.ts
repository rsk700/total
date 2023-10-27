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

export async function startScan(path: string): Promise<ms.structs.None> {
    return ipcRequest("start_scan", path, t.str, t.None);
}

export async function scanStep(timeBudgetMs: number): Promise<ms.enums.ScanState> {
    return ipcRequest("scan_step", timeBudgetMs, t.int, t.ScanState);
}

export async function getAggregateData(upToFraction: number): Promise<ms.structs.AggregateEntry[]> {
    return ipcRequest("get_aggregate_data", upToFraction, t.float, [t.list, t.AggregateEntry]);
}

export async function openPath(path: String): Promise<ms.structs.None> {
    return ipcRequest("open_path", path, t.str, t.None);
}