import oasis from '@oasislabs/client';

jest.setTimeout(20000);

describe('TypeX Test', () => {
  let service;

  beforeAll(async () => {
    service = await oasis.workspace.TypeX.deploy({
      header: { confidential: false },
      gasLimit: '0xe79732',
    });
  });

  it('deployed', async () => {
    expect(service).toBeTruthy();
  });

  afterAll(() => {
    oasis.disconnect();
  });
});
