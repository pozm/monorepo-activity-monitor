
export interface DefaultModel {
    ID: number;
    CreatedAt: Date;
    UpdatedAt: Date;
    DeletedAt: null | Date;

}

export interface Activity extends DefaultModel {
    UserDataID: number;
    name: string;
    minsTotal: number;
    active: boolean;
}

export interface User extends DefaultModel {
    name: string;
    activities: Activity[];
}


