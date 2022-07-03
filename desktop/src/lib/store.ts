import { fs, invoke } from "@tauri-apps/api";
import { appDir } from "@tauri-apps/api/path";
import type { IDataStoreTypes } from "./DataTypes";

const strToAB = (str) =>
    new Uint8Array(str.split("").map((c) => c.charCodeAt(0)));

const ABToStr = (ab) =>
    new Uint8Array(ab).reduce((p, c) => p + String.fromCharCode(c), "");

const DataUrl = async () => `${await appDir()}.data/pog/.pog`;

export namespace Data {


    export class Store {
        private _data: IDataStoreTypes
		private lastWrite : IDataStoreTypes;
        private static instance: Store = new Store();
        private constructor () {
            read().then(dat=>this._data=dat)
        };
        static get get() {return this.instance};

		get data() {
			return this._data
			// return this.ProxyDatafn(this._data) as IDataStoreTypes | undefined
		}
        // async clear(){
        //     this.data = {};
        //     await write(this.data);
        // }
        // async add(key:string,val: string) {
        //     this.data[key] = val
        //     await write(this.data);
        // }
        async write() {
			if (this.lastWrite === this._data) return console.log("same",this._data,this.lastWrite);
			this.lastWrite = {...this._data};
            await write(this._data);
        }
        async reload() {
            return await read().then(dat=>(this._data=dat,dat));
        }
        // async del(site:string) {
        //     delete this.data[site];
        //     await write(this.data);
        // }
    }

    function read() {
		return invoke<IDataStoreTypes>("get_data").then(data=>{
			console.log("READ",data)
			
			return data;
		},e=>e)
    }
    function write(data: IDataStoreTypes) {
		console.log("WRITE",data)
        return invoke("set_data",{new : data})
    }
}
