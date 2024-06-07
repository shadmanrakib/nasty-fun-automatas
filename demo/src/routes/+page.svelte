<script lang="ts">
  import init, { Regex } from "$lib/nasty-fun-automatas/nasty_fun_automatas";
  import { presets } from "$lib/presets";
  import { onMount } from "svelte";

  let regexInput = "";
  let testCases: { text: string; matches: boolean }[] = [];

  function runTests() {
    try {
      const regex = Regex.new(regexInput);
      testCases = testCases.map((testCase) => ({
        ...testCase,
        matches: regex?.isMatch(testCase.text) || false,
      }));
    } catch (error) {
      //   alert("Invalid regex");
      testCases = testCases.map((testCase) => ({
        ...testCase,
        matches: false,
      }));
    }
  }

  type OnChangeInputEvent = Event & {
    currentTarget: EventTarget & HTMLInputElement;
  };

  function handleTestCaseInput(event: OnChangeInputEvent, index: number) {
    testCases[index].text = event.currentTarget.value;
    runTests();
  }

  function handleRegexInput(event: OnChangeInputEvent) {
    regexInput = event.currentTarget.value;
    runTests();
  }

  function selectPreset(preset: (typeof presets)[0]) {
    regexInput = preset.regex;
    testCases = preset.cases;
    runTests();
  }

  function addTestCase() {
    testCases = [...testCases, { text: "", matches: false }];
  }

  function removeTestCase(index: number) {
    testCases = testCases.filter((_, i) => i !== index);
    runTests();
  }

  onMount(() => {
    init().then((v) => {
      runTests();
    });
  });

  selectPreset(presets[0]);
</script>

<main
  class="flex flex-col md:flex-row p-6 bg-gray-950 text-gray-100 min-h-screen"
>
  <aside class="md:w-1/4 p-6 bg-white/5 rounded-lg shadow-md">
    <h2 class="text-xl font-bold mb-4 text-blue-200">About Regex Language</h2>
    <p class="text-gray-200">
      Regular expressions are patterns used to match character combinations in
      strings. They are used in search engines, text processing utilities, and
      more.
    </p>
    <ul class="list-disc list-inside mt-4">
      <li>
        <code class="bg-gray-800">.</code> - Matches any single character.
      </li>
      <li>
        <code class="bg-gray-800">*</code> - Matches zero or more occurrences of
        the preceding element.
      </li>
      <li>
        <code class="bg-gray-800">+</code> - Matches one or more occurrences of the
        preceding element.
      </li>
      <li>
        <code class="bg-gray-800">?</code> - Matches zero or one occurrence of the
        preceding element.
      </li>
      <li><code class="bg-gray-800">|</code> - Alternation operator (or).</li>
      <li><code class="bg-gray-800">[]</code> - Character class.</li>
      <li>
        <code class="bg-gray-800">\\</code> - Escape special characters.
      </li>
    </ul>
  </aside>
  <section class="md:w-3/4 md:ml-6 mt-6 md:mt-0">
    <div class="flex justify-between items-center mb-4">
      <h1 class="text-3xl font-bold">Nasty Fun Automatas Regex Demo</h1>
    </div>

    <div class="mb-4 flex items-center">
      <label for="presets" class="block text-lg font-mediumtext-gray-300 pr-4"
        >Presets:</label
      >
      <div class="flex flex-wrap gap-2 mt-2">
        {#each presets as preset}
          <button
            on:click={() => selectPreset(preset)}
            class="bg-gray-700 text-white px-4 py-2 rounded hover:bg-blue-600"
            >{preset.name}</button
          >
        {/each}
      </div>
    </div>

    <div class="mb-4">
      <label for="regex" class="block text-lg font-medium text-gray-300"
        >Regular Expression:</label
      >
      <input
        id="regex"
        type="text"
        bind:value={regexInput}
        on:input={handleRegexInput}
        class="mt-1 p-2 border rounded w-full bg-gray-800 border-gray-700 text-gray-100"
      />
    </div>

    <div class="mb-4">
      <h2 class="text-2xl font-bold mb-4">Test Cases</h2>
      {#each testCases as testCase, index}
        <div class="flex items-center">
          <div class="w-full max-w-2xl relative pr-8">
            <input
              type="text"
              bind:value={testCase.text}
              on:input={(e) => handleTestCaseInput(e, index)}
              class="p-2 border rounded w-full bg-gray-800 border-gray-700 text-gray-100"
            />
            <button
              on:click={() => removeTestCase(index)}
              class="absolute z-10 right-9 top-1 bg-gray-900 text-white px-2 py-1 rounded hover:bg-red-600"
            >
              delete
            </button>
          </div>

          {#if testCase.matches}
            <p class="text-green-500 w-24 px-2">Match</p>
          {:else}
            <p class="text-red-500 w-24 px-2">Mismatch</p>
          {/if}
        </div>
      {/each}
      <button
        on:click={addTestCase}
        class="mt-4 bg-blue-700 text-white px-4 py-2 rounded hover:bg-blue-500"
        >Add Test Case</button
      >
    </div>
  </section>
</main>
