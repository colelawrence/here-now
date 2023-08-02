// @ts-check
const launchEditor = require("launch-editor")
const [file] = process.argv.slice(2)
console.error("launchEditor", { file })
launchEditor(file, err => {
  console.error(`Failed to open ${file}`, err)
  process.exit(1)
})
