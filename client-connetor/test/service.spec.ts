import oasis from '@oasislabs/client';

jest.setTimeout(200000);

describe('TypeX Test', () => {

let service;
let routerService;
let userService;

beforeAll(async () => {
    service = await oasis.workspace.TypeX.deploy({
      header: { confidential: false },
      gasLimit: '0xe92732',
    });
	
	//let address1 = '0x441d4370976cbee0aef9c6e206b119e818002de1';
	
	
	routerService = await oasis.workspace.Router.deploy({
      header: { confidential: false },
      gasLimit: '0xe92732',
    });
});

it('deployed', async () => {
	//let hex = Buffer.from(service.address).toString('hex')
	//console.log(hex);
	
    //expect(service).toBeTruthy();
	expect(routerService).toBeTruthy();
	
	//let hex1 = Buffer.from(routerService.address).toString('hex')
	//console.log(hex1);
});

it('Test: set the user contract address', async () => {
	let addr = '0x' + Buffer.from(service.address).toString('hex');
	//console.log('0x' + addr);
    let rs1 = await routerService.setUserContractAddress('test',addr);
    //expect(true).toEqual(rs);
});

let usercontractaddress;
it('Test: get the user contract address', async () => {
    let usercontractaddress = await routerService.getUserContractAddress('test');
    console.log(usercontractaddress);
});


it('Test: get the user service/handle', async () => {
	//let newaddr = usercontractaddress+ '';
	//console.log(newaddr);
	userService = await oasis.Service.at(Buffer.from(service.address).toString('hex'));
	console.log(userService);
    //expect(true).toEqual(rs);
});
/*
it('Test: register the new users', async () => {
    let rs2 = await userService.registerUser('typexs');
});

it('Test: insert the public parameters', async () => {
    let rs0 = await userService.registerParam(['304583', '30458']);
    //expect(true).toEqual(rs);
});
*/

 //it('Test: prepare assumption data', async () => {
 //   let rs1 = await service.prepareAssumptionData();
    //expect(true).toEqual(rs);
// });


//let rs3;
//it('Test: get proof for the user', async () => {
//    rs3 = await service.getProof('typexs');
//});

 afterAll(() => {
   //console.log(rs3);
   oasis.disconnect();
 });
});
