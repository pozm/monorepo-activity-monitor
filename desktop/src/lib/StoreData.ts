import { writable } from "svelte/store";
import type { IApplication, IClientNotification } from "./DataTypes";
export const AppNotifications = writable<Map<number,IClientNotification>>(new Map())


export const ShowNewModal = writable(false)
export const Applications = writable<(IApplication & {open?:boolean,dateOpen?:Date})[]>([])

export const ShowServerModal = writable(false)
export const ServerModalType = writable(0)
export const ServerAddress = writable("")
export const ServerKey = writable("")