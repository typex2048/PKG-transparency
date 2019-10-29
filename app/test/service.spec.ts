import oasis from '@oasislabs/client';

 jest.setTimeout(20000);

 describe('TypeX Test', () => {
  let service;

 beforeAll(async () => {
    service = await oasis.workspace.TypeX.deploy({
      header: { confidential: false },
      gasLimit: '0xebced8',
    });
 });

 it('deployed', async () => {
    expect(service).toBeTruthy();
 });

 it('Test: insert the public parameters', async () => {
    let rs0 = await service.registerParam(['30458', '30458']);
    //expect(true).toEqual(rs);
 });

 it('Test: prepare assumption data', async () => {
    let rs1 = await service.prepareAssumptionData();
    //expect(true).toEqual(rs);
 });

 it('Test: register the new users', async () => {
    let rs2 = await service.registerUser('typexs');
 });

 it('Test: get proof for the user', async () => {
    let rs3 = await service.getProof('typexs');
 });

 afterAll(() => {
   oasis.disconnect();
 });
});
