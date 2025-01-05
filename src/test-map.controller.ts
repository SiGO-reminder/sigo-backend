import { Controller, Get, Query } from '@nestjs/common'; // @Query로 쿼리 파라미터를 받을 수 있음
import { NaverMapService } from './naver-map/naver-map.service'; // 우리가 만든 서비스 가져오기

@Controller('map') // '/map' 경로에 대한 요청 처리
export class TestMapController {
  constructor(private readonly naverMapService: NaverMapService) {}

  @Get('geocode') // '/map/geocode'로 GET 요청을 처리
  async getGeocode(@Query('address') address: string) {
    if (!address) {
      return { message: '주소를 입력하세요! 😊', example: '/map/geocode?address=서울특별시 중구 세종대로 110' };
    }
    try {
      const result = await this.naverMapService.getGeocode(address);
      return { message: '성공적으로 좌표를 가져왔어요!', data: result };
    } catch (error) {
      return { message: '오류가 발생했어요 😢', error: error.message };
    }
  }
}
