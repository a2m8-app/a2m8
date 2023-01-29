/** @jsxImportSource preact */
import type { FunctionalComponent } from "preact";
import { useState, useEffect, useRef } from "preact/hooks";
import { GITEA } from "../consts";
import { gitea } from "../lib/gitea";

const AVAILABLE_TAGS = ["utility", "keybindings", "automation", "ui", "misc"];
function createCustomSorter(name: string) {
  return function customSorter(a: any, b: any) {
    let prioList = [
      `${name}.lua`,
      "mod.lua",
      "index.lua",
      "main.lua",
      "program.lua",
      "script.lua",
    ];
    let aPrio = prioList.indexOf(a.name);
    let bPrio = prioList.indexOf(b.name);
    if (aPrio !== -1 && bPrio !== -1) {
      return aPrio - bPrio;
    }
    if (aPrio !== -1) {
      return -1;
    }
    if (bPrio !== -1) {
      return 1;
    }

    return a.name.localeCompare(b.name);
  };
}

const WorkShop = () => {
  let [tagFilters, setTagFilters] = useState<string[]>(["script"]);
  let [data, setData] = useState<any>([]);
  let [virtualPage, setVirtualPage] = useState(0);
  let [loading, setLoading] = useState(false);
  let [isEnd, setIsEnd] = useState(false);
  let observerRef = useRef<IntersectionObserver | null>(null);
  let loadingRef = useRef(null);
  useEffect(() => {
    observerRef.current = new IntersectionObserver((entries) => {
      entries.forEach((entry) => {
        if (entry.isIntersecting) {
          setLoading(true);
          fetchMore().then(() => setLoading(false));
          observerRef.current!.unobserve(entry.target);
        }
      });
    });
    //@ts-ignore -
    observerRef.current.observe(loadingRef.current);
    fetchMore(20);
  }, []);

  const fetchMore = async (count?: number) => {
    let url = new URL("https://a2m8-git.tricked.dev/api/v1/repos/search");
    url.searchParams.set("q", tagFilters.join(" "));
    url.searchParams.set("page", virtualPage.toString());
    url.searchParams.set("per_page", count?.toString() ?? "10");
    url.searchParams.set("topic", "true");
    let results = await fetch(url).then((r) => r.json());
    if (results.ok) {
      let res = await Promise.all(
        results.data
          .map((x: any) => ({
            name: x.name,
            description: x.description,
            full_name: x.full_name,
            stars_count: x.stars_count,
            dev: x.owner.login,
            avatar: x.avatar_url ?? x.owner.avatar_url,
          }))
          .map(async (x: any) => {
            const data = await fetch(
              "https://a2m8-git.tricked.dev/api/v1/repos/" +
                x.full_name +
                "/topics"
            ).then((r) => r.json());
            return {
              topics: data.topics,
              ...x,
            };
          })
      );
      if (res.length < 10) {
        setIsEnd(true);
      }

      setData([...data, ...res]);
      setVirtualPage(virtualPage + 1);
    }
  };

  return (
    <>
      <div class="flex flex-wrap gap-2 px-2 py-4 justify-center">
        {AVAILABLE_TAGS.map((tag) => (
          <button
            key={tag}
            class={`${
              tagFilters.includes(tag) ? "btn-primary" : ""
            } btn btn-sm`}
            onClick={() => {
              if (tagFilters.includes(tag)) {
                setTagFilters(tagFilters.filter((t) => t !== tag));
              } else {
                setTagFilters([...tagFilters, tag]);
              }
              setData([]);
              setVirtualPage(0);
              setIsEnd(false);
              fetchMore();
            }}
          >
            {tag}
          </button>
        ))}
      </div>
      <div class="mb-8 grid justify-center">
        <div class="grid flex-wrap mx-auto md:grid-cols-4 sm:grid-cols-2 justify-center gap-2 max-w-[70rem]">
          {data.map((item: any) => (
            <div
              key={item.full_name}
              class="card p-0 hover:scale-105 transition-transform duration-150 hover:shadow-md hover:shadow-primary/5"
            >
              <figure class="image-wrapper">
                <img src={item.avatar || "/A2.png"} class="w-1/3" />
              </figure>
              <div class="card-body p-4 bg-base-300 rounded-sm">
                <h2 class="card-title mb-0 !mt-0">{item.name}</h2>
                <p class="text-sm text-base-content/80">
                  By <a href={`${GITEA}/${item.dev}`}>{item.dev}</a>
                </p>
                <p class="card-subtitle">{item.description}</p>
                <div class="flex flex-wrap gap-2">
                  {item.topics
                    .filter((topic: string) => topic !== "script")
                    .map((topic: string) => (
                      <span
                        key={topic}
                        class="badge badge-secondary badge-sm px-0"
                      >
                        {topic}
                      </span>
                    ))
                    .slice(0, 4)}
                </div>
                <div class="flex flex-wrap gap-2">
                  <button
                    onClick={async () => {
                      let contents = await gitea.repos
                        .repoGetContentsList(item.dev, item.name)
                        .then((r) => r.data);
                      let file = contents
                        .filter((x) => x.name?.endsWith(".lua"))
                        .sort(createCustomSorter(item.name))[0];
                      if (file) {
                        let data = await gitea.repos
                          .repoGetContents(item.dev, item.name, file.path!)
                          .then((r) => r.data);
                        if (data) {
                          let decoded = atob(data.content!);
                          let ok = await fetch(
                            `http://127.0.0.1:5836/new?${item.name}.lua`,
                            {
                              method: "POST",
                              body: decoded,
                            }
                          ).then((r) => r.ok);
                          if (!ok) {
                            alert("Failed to open installer!");
                          }
                        }
                      }
                    }}
                    class="btn btn-xs btn-primary"
                  >
                    Install
                  </button>
                  <a
                    href={`${GITEA}/${item.full_name}`}
                    class="btn btn-xs btn-secondary"
                  >
                    View
                  </a>
                </div>
              </div>
            </div>
          ))}
        </div>
        <div
          ref={loadingRef}
          class={`btn btn-ghost mx-auto text-center ${isEnd ? "" : ""}`}
        >
          {loading && !isEnd && <div>Loading...</div>}
          {loading && isEnd && <div>End of the list</div>}
        </div>
      </div>
    </>
  );
};
export default WorkShop;
