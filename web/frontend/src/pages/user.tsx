import { formatDistanceToNow, formatDuration, intervalToDuration, sub } from "date-fns";
import { Link, useParams } from "solid-app-router";
import { createMemo, createResource, For, Match, Show, Suspense, Switch } from "solid-js";
import { base_path } from "../../config";
import { Activity, User } from "../types";

const fetchUser = async (id:string) =>
(await fetch(`${base_path}api/user/${id}`,{credentials:"omit",mode:"cors"})).json() as Promise<User>;


export default function UserPage() {
    const params = useParams();
    const [user] = createResource(params.name, fetchUser);

    const getTimeSpent = (activity:Activity)=>{
        let totmins = 0;
        {
            let since = 0;
            if (activity.active) {
                let past = new Date().getTime() - new Date(activity.updated_at as unknown as string).getTime();
                since = Math.floor(past/1000/60)
            }
            totmins = activity.mins_total + since
        }
        let dist = sub(new Date(),{minutes:activity.mins_total+totmins})
        let dur = intervalToDuration({start:dist,end:new Date()})
        console.log(dur)
        let dur2 = formatDuration(dur,{
            format:["years","months","weeks","days","hours","minutes"]
        });
        return dur2.trim() == "" ? "0 minutes" : dur2;
    }
    const getLastActive = (activity:Activity)=>{
        if (activity?.active) {
            return "Now"
        }
        let updated =  new Date(activity.updated_at as unknown as string)
        return `${formatDistanceToNow(updated,{addSuffix:true})} (${updated.toLocaleDateString(navigator.language,{dateStyle:"medium"})})`
    }

    return (
        <div class="flex flex-col  p-20 text-gray-300">
            <Suspense fallback={"loading..."} >

            <Switch fallback={<div>User Not Found</div>} >
                <Match when={user()?.name} >
                    <img src="" />
                    <h1 class="text-gray-200 font-semibold text-5xl first-letter:uppercase" >{`${user()?.name ?? "unknown"}'s`}</h1>
                    <h3 class="font-medium text-xl pb-4" >Activities</h3>

                    <div class="flex w-full flex-wrap flex-row justify-around" >
                        <For each={Object.entries(user()?.activities ?? {})}>{([aname,activity]) => 
                            <div class="mb-2 border-solid rounded-xl border-zinc-700 border-2 p-5 bg-zinc-900 shadow-xl flex flex-col" >
                                <img src={`${base_path}api/user/${user()?.name}/img/${aname}`} class="self-center" width="64" />
                                <h2 class="text-xl font-medium first-letter:uppercase self-center" >
                                    {aname}
                                </h2>
                                <p>Time Spent: <span class="text-pink-300 w-full " >{getTimeSpent(activity)}</span></p>
                                <p>Active Now : <span class="text-pink-300" >{activity.active ? "Yes" : "No"}</span></p>
                                <p>Last Active : <span class="text-pink-300" >{getLastActive(activity)}</span></p>
                                <div class="flex flex-wrap space-x-2" >
                                    <p>Devices :</p> <li class="flex list-none">
                                        <For each={user()?.devices.filter(v=>activity.devices?.includes(v.deviceId))}>{d=><li class="comma"><Link class="text-pink-300" href={`./d/${d.name}`} >{d.name}</Link></li>}</For>
                                    </li>
                                    
                                </div>
                            </div>
                        }</For>
                    </div>
                </Match>
            </Switch>

            </Suspense>
        </div>
    )
}