import { formatDuration, intervalToDuration, sub } from "date-fns";
import { useParams } from "solid-app-router";
import { createMemo, createResource, For } from "solid-js";
import { base_path } from "../../config";
import { Activity, User } from "../types";

const fetchUser = async (id:string) =>
(await fetch(`${base_path}api/user/${id}/`,{credentials:"omit",mode:"cors"})).json() as Promise<User>;


export default function UserPage() {
    const params = useParams();
    const [user] = createResource(params.name, fetchUser);

    const getTimeSpent = (activity:Activity)=>{
        let totmins = 0;
        {
            let since = 0;
            if (activity.active) {
                let past = new Date().getTime() - new Date(activity.UpdatedAt as unknown as string).getTime();
                since = Math.floor(past/1000/60)
            }
            totmins = activity.minsTotal + since
        }
        let dist = sub(new Date(),{minutes:activity.minsTotal+totmins})
        let dur = intervalToDuration({start:dist,end:new Date()})
        console.log(dur)
        return formatDuration(dur,{
            format:["years","months","days","weeks","days","minutes"]
        })
    }

    return (
        <div class="flex flex-col  p-20 text-gray-300">
            <img src="" />
            <h1 class="text-gray-200 font-semibold text-5xl first-letter:uppercase" >{user()?.name}'s</h1>
            <h3 class="font-medium text-xl" >Activities</h3>

            <div class="flex w-full flex-wrap flex-row justify-around" >
                <For each={user()?.activities}>{activity => 
                    <div class="mb-2 border-solid rounded-xl border-zinc-700 border-2 p-5 bg-zinc-900 shadow-xl flex flex-col" >
                        <img src={`${base_path}api/user/${user()?.name}/img/${activity.name}`} class="self-center" width="64" />
                        <h2 class="text-xl font-medium first-letter:uppercase self-center" >
                            {activity.name}
                        </h2>
                        <p>Time Spent: <span class="text-pink-300 w-full " >{getTimeSpent(activity)}</span></p>
                        <p>Active Now : <span class="text-pink-300" >{activity.active ? "Yes" : "No"}</span></p>
                    </div>
                }</For>
            </div>
        </div>
    )
}