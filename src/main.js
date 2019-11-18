const lib = require("./index.js");

async function main() {
  const periods = await lib.list(35.7796,-78.6382);
  const best = lib.get_best(periods);
  const now = lib.get_now(periods);

  if( best === now ) {
    console.log(lib.human(best)+"\n"); // eslint-disable-line no-console
  } else {
    //console.log(lib.human(best)+"\n"); // eslint-disable-line no-console
  }

  //console.log("\nAlso:\n"); // eslint-disable-line no-console

  //for( const period of periods ) {
  //  console.log(lib.human(period)+"\n"); // eslint-disable-line no-console
  //}
}

main();
