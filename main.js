import init, { run_app } from './pkg/sso_page.js';
async function main() {
   await init('/pkg/sso_page_bg.wasm');
   run_app();
}
main()