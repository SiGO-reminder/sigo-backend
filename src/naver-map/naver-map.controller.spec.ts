import { Test, TestingModule } from '@nestjs/testing';
import { NaverMapController } from './naver-map.controller';

describe('NaverMapController', () => {
  let controller: NaverMapController;

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      controllers: [NaverMapController],
    }).compile();

    controller = module.get<NaverMapController>(NaverMapController);
  });

  it('should be defined', () => {
    expect(controller).toBeDefined();
  });
});
