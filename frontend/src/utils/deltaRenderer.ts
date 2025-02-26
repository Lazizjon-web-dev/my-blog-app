import { Quill, type Delta } from "@vueup/vue-quill";
import { sanitizeHTML } from "./sanitizeHtml";

/**
 * Converts a Delta object to HTML.
 * @param {Delta} delta - The Delta object (array of operations).
 * @returns {string} - The rendered HTML as a string.
 */
export const deltaToHTML = (delta: Delta): string => {
    try {
        const quill = new Quill(document.createElement("div"));
        quill.setContents(delta);
        return quill.root.innerHTML;
    } catch (error) {
        console.log("Error rendering Delta content:", error);
        return "<p>Error rendering content.</p>";
    }
};

/**
 * Converts a Delta object to plain text.
 * @param {Delta} delta - The Delta object (array of operations).
 * @returns {string} - The plain text representation of the Delta.
 */
export const deltaToText = (delta: Delta): string => {
    if (!Array.isArray(delta)) {
        throw new Error("Input must be a Delta array.");
    }

    let text = "";

    delta.forEach((op) => {
        if (typeof op.insert === "string") {
            text += op.insert;
        }
    });
    return text;
};

/**
 * Renders Delta content as sanitized HTML.
 * @param delta - The Delta content as a JSON string.
 * @returns The sanitized HTML as a string.
 */
export const renderDelta = (delta: string): string => {
    try {
        const html = deltaToHTML(JSON.parse(delta));
        return sanitizeHTML(html);
    } catch (error) {
        console.log("Error rendering Delta content:", error);
        return "<p>Error rendering content.</p>";
    }
};
