const create_r = (key: any, map: Map<string, string>): string => {
  return map.has(key) ? key + create_r(map.get(key), map) : key;
};
function word(list: string[]): string {
  let proc_list: string[][] = list.map((e) => {
    return e.split(">");
  });

  let map_for = new Map();
  let map_back = new Map();
  proc_list.forEach((e) => {
    map_for.set(e[0], e[1]);
    map_back.set(e[1], e[0]);
  });
  return (
    create_r(proc_list[Math.floor(list.length / 2)][0], map_back).split("").reverse().join("") +
    create_r(proc_list[Math.floor(list.length / 2)][1], map_for)
  );
}

console.log(word(["S>P", "P>A", "A>I", "I>N"]));
