import { Activity, Star } from "lucide-react";
import Link from "@/ui/link";
import { timeAgo } from "@/util";

type StarActivity = {
  id: string;
  userName: string;
  date: Date;
};

const MIN = 60 * 1000;
const HR = 60 * MIN;
const DAY = 24 * HR;

const PLACEHOLDER_ACTIVITY: StarActivity[] = [
  {
    id: "1",
    userName: "alice-chen",
    date: new Date(Date.now() - 12 * MIN),
  },
  {
    id: "3",
    userName: "marcus-aurelius",
    date: new Date(Date.now() - 2 * HR),
  },
  {
    id: "6",
    userName: "yuki-tanaka",
    date: new Date(Date.now() - 8 * HR),
  },
  {
    id: "7",
    userName: "priya-patel",
    date: new Date(Date.now() - 14 * HR),
  },
  {
    id: "10",
    userName: "sofia-rossi",
    date: new Date(Date.now() - DAY - 6 * HR),
  },
  {
    id: "13",
    userName: "amara-okafor",
    date: new Date(Date.now() - 3 * DAY),
  },
  {
    id: "15",
    userName: "noah-fischer",
    date: new Date(Date.now() - 4 * DAY),
  },
  {
    id: "17",
    userName: "hana-suzuki",
    date: new Date(Date.now() - 6 * DAY),
  },
  {
    id: "18",
    userName: "leo-bauer",
    date: new Date(Date.now() - 6 * DAY - 5 * HR),
  },
  {
    id: "20",
    userName: "isabela-cruz",
    date: new Date(Date.now() - 9 * DAY),
  },
  {
    id: "22",
    userName: "theo-wagner",
    date: new Date(Date.now() - 13 * DAY),
  },
  {
    id: "24",
    userName: "fatima-nasser",
    date: new Date(Date.now() - 18 * DAY),
  },
];

export function RepoActivity() {
  return (
    <div className="flex-1 min-h-0 flex flex-col p-2">
      <span className="flex items-center gap-1.5 text-xs text-muted-foreground font-mono mb-3">
        <Activity className="size-3.5" />
        Activity
      </span>
      <div className="flex flex-col gap-2 overflow-y-auto scrollbar-none">
        {PLACEHOLDER_ACTIVITY.map((item) => (
          <ActivityRow key={item.id} item={item} />
        ))}
        {PLACEHOLDER_ACTIVITY.length === 0 && (
          <span className="font-mono text-xs text-muted-foreground">
            no activity
          </span>
        )}
      </div>
    </div>
  );
}

function ActivityRow({ item }: { item: StarActivity }) {
  return (
    <div className="flex flex-col min-w-0 text-xs">
      <div className="truncate">
        <Link
          href={`/${item.userName}`}
          className="font-medium hover:underline"
        >
          {item.userName}
        </Link>
        <span className="text-muted-foreground"> starred</span>
      </div>
      <div className="flex items-center gap-1 text-[10px] font-mono text-muted-foreground">
        <Star className="size-3 shrink-0" />
        <span>{timeAgo(item.date)}</span>
      </div>
    </div>
  );
}
