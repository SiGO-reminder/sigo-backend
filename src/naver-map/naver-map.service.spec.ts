import { Test, TestingModule } from '@nestjs/testing';
import { NaverMapService } from './naver-map.service';

describe('NaverMapService', () => {
  let service: NaverMapService;

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      providers: [NaverMapService],
    }).compile();

    service = module.get<NaverMapService>(NaverMapService);
  });

  it('should be defined', () => {
    expect(service).toBeDefined();
  });
});
