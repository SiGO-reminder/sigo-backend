import { Controller, Get, Query } from '@nestjs/common'; // HTTP 요청 핸들링을 위해 필요한 데코레이터
import { NaverMapService } from './naver-map.service'; // 서비스 연결

@Controller('naver-map') // 이 컨트롤러의 URL 기본 경로를 지정 (예: http://localhost:3000/naver-map)
export class NaverMapController {
  constructor(private readonly naverMapService: NaverMapService) {} // 서비스 주입

  @Get('/geocode') // HTTP GET 요청을 처리하는 메서드
  async getGeocode(@Query('address') address: string) {
    // 주소를 서비스로 전달하여 결과를 받아옴
    const result = await this.naverMapService.getGeocode(address);
    return result; // 클라이언트에게 API 응답 데이터를 반환
  }
}
