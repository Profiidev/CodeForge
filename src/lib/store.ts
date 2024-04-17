import { writable } from "svelte/store";

export let style = writable({
  tokenColors: {
    none: "#000000",
    comment: "#000000",
    string: "#000000",
    number: "#000000",
    variable: "#000000",
    member_variable: "#000000",
    keyword: "#000000",
    operator: "#000000",
    punctuation: "#000000",
    function_name: "#000000",
    function_call: "#000000",
    function_argument: "#000000",
    separator: "#000000",
    import: "#000000",
    entity_name: "#000000",
    tag: "#000000",
    annotation: "#000000",
    invalid: "#000000",
    type: "#000000",
    bracket: "#000000",
    brace: "#000000",
    parenthesis: "#000000",
    attribute: "#000000",
  },
});