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

  it('insert the public parameters', async () => {
    let rs = await service.registerParam(['30458', '30458']);
    //expect(true).toEqual(rs);
  });

  afterAll(() => {
    oasis.disconnect();
  });
});
