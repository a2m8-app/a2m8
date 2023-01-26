/** @jsxImportSource preact */
import type { FunctionalComponent } from "preact";
import { useState, useEffect, useRef } from "preact/hooks";

const AVAILABLE_TAGS = ["utility", "keybindings", "automation", "ui", "misc"];

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
      <div class="w-full bg-base-200 py-7 pl-10">
        <h1 class="text-xl text-">Workshop</h1>
        <p>Download and use Script from here</p>
      </div>
      <div class="flex flex-wrap gap-2 px-2 py-4 justify-center">
        {AVAILABLE_TAGS.map((tag) => (
          <button
            class={`${
              tagFilters.includes(tag) ? "btn-primary" : ""
            } btn btn-sm`}
            onClick={() => {
              if (tagFilters.includes(tag)) {
                setTagFilters(tagFilters.filter((t) => t !== tag));
              } else {
                setTagFilters([...tagFilters, tag]);
              }
            }}
          >
            {tag}
          </button>
        ))}
      </div>
      <div class="mb-8">
        <div class="flex flex-wrap mx-auto grid-cols-3 justify-center gap-2">
          {data.map((item) => (
            <div class="card bg-base-300 p-0">
              <div class="card-body p-2 rounded-sm">
                <figure>
                  <img src={item.avatar} class="aspect-video" alt="Shoes" />
                </figure>
                <h2 class="card-title mb-0 !mt-0">{item.name}</h2>
                <p class="text-sm text-base-content/80">
                  By <a href={`__GITEA__/${item.dev}`}>{item.dev}</a>
                </p>
                <p class="card-subtitle">{item.description}</p>
                <div class="flex flex-wrap gap-2">
                  {item.topics.map((topic: string) => (
                    <span class="badge badge-secondary badge-sm">{topic}</span>
                  ))}
                </div>
                <div>
                  <a class="btn btn-xs btn-primary">Install</a>
                </div>
              </div>
            </div>
          ))}
        </div>
        <div ref={loadingRef}>
          {loading && !isEnd && <div>Loading...</div>}
          {loading && isEnd && <div>End of the list</div>}
        </div>
      </div>
    </>
  );
};
export default WorkShop;
