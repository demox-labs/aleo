const ALEO_URL = 'http://localhost:3030/testnet3';
const STATE_ROOT_URL = `${ALEO_URL}/latest/stateRoot`;
const STATE_PATH_URL = `${ALEO_URL}/statePath`;

export async function getStateRoot() {
  const response = await fetch(STATE_ROOT_URL);
  const root = await response.text();
  return root.substring(1, root.length - 1);
}

export async function getStatePath(commitment) {
  const response = await fetch(`${STATE_PATH_URL}/${commitment}`);
  const path = await response.text();
  return path.substring(1, path.length - 1);
}

export async function inputIdsToStatePathMap(inputIdsString) {
  const inputIds = JSON.parse(inputIdsString);
  const commitments = inputIds.filter(inp => inp.commitment).map(inp => inp.commitment);
  const statePaths = {
    map: {}
  };
  await Promise.all(commitments.map(async commitment => {
    const statePath = await getStatePath(commitment);
    statePaths.map[commitment] = statePath;
  }));

  return statePaths;
}