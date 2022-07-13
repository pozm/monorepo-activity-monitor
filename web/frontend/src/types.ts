
export interface DefaultModel {
    ID: number;
    CreatedAt: Date;
    UpdatedAt: Date;
    DeletedAt: null | Date;

}

export interface Activity {
    activity_id: number;
    devices: {[x:string]:number};
    mins_total: number;
    updated_at: Date;
    created_at: Date;
    active: boolean;
}

export interface User {
    name: string;
    activities: {[x:string]:Activity};
    devices: Device[]
}

export interface Device {
    ID: number;
    CreatedAt: Date;
    UpdatedAt: Date;
    DeletedAt?: any;
    UserDataID: number;
    name: string;
    deviceId: string;
}
