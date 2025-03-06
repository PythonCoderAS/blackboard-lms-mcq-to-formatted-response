import { invoke } from "@tauri-apps/api/tauri";
import "bootstrap/dist/css/bootstrap.min.css";

async function appLoop() {
  const text =
    document.querySelector<HTMLTextAreaElement>("#text-input")?.value;
  if (!text?.trim()) {
    return alert("Please enter some non-whitespace text.");
  }
  const numAnswers = Number(
    document.querySelector<HTMLInputElement>("#num-answers-input")?.value
  );
  if (isNaN(numAnswers) || numAnswers < 1) {
    return alert("Please enter a number of answers.");
  } else if (numAnswers > 26) {
    return alert("Please enter a number of answers less than 27.");
  }
  const allowMultipleChoices = document.querySelector<HTMLInputElement>(
    "#allow-multiple-input"
  )?.checked;
  if (allowMultipleChoices === null) {
    return alert("Please select whether multiple choices are allowed.");
  }
  const response: string = await invoke(
    "tauri_transform_text_to_formatted_string",
    {
      text,
      numChoices: numAnswers,
      multipleChoicesAllow: allowMultipleChoices,
    }
  );
  document.querySelector("#output")!.textContent = response;
  document.querySelector<HTMLTextAreaElement>("#output")?.select();
}

window.addEventListener("load", () => {
  document.querySelector("#submit")!.addEventListener("click", appLoop);
});
