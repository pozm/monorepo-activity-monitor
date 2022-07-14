export interface IClientNotification {
	title:string
	data:string,
	html?:boolean
	err?:boolean
}


export interface IDataStoreTypes {
    server:IServerSettings
}

export interface IServerSettings {
    address:string,
    api_key:string
    applications : IApplication[],
}

export interface IApplication {
	location:string,
	name:string,
	icon_location:string,
}
export module GithubTagApi {

    export interface Object {
        sha: string;
        type: string;
        url: string;
    }

    export interface RootObject {
        ref: string;
        node_id: string;
        url: string;
        object: Object;
    }

}

