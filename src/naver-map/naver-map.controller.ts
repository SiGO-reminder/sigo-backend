import { Controller, Get, Query, Post, Body } from '@nestjs/common'; // HTTP 요청 핸들링을 위해 필요한 데코레이터
import { NaverMapService } from './naver-map.service'; // 서비스 연결

@Controller('naver-map') // 이 컨트롤러의 URL 기본 경로를 지정
export class NaverMapController {
  constructor(private readonly naverMapService: NaverMapService) {} // 서비스 주입

  // GET 요청: 쿼리 파라미터를 통해 장소명을 전달받아 좌표를 반환
  @Get('/search') 
  async searchPlace(@Query('address') address: string) {
    if (!address) {
      return { 
        message: '주소를 입력하세요! 😊',
        example: 'http://localhost:3000/naver-map/geocode?address=서울시청',
      };
    }
    try {
      const result = await this.naverMapService.getCoordinatesFromKeyword(address);
      return { 
        message: '장소 정보를 성공적으로 가져왔어요!', 
        data: result,
      };
    } catch (error) {
      return { 
        message: '오류가 발생했어요 😢', 
        error: error.message,
      };
    }
  }

  // POST 요청: JSON 형식으로 장소명을 전달받아 좌표를 반환
  @Post('/search') 
  async postCoordinatesFromKeyword(@Body('address') address: string) {
    if (!address) {
      return { 
        message: '주소를 입력하세요! 😊',
        example: 'Body에 { "address": "서울시청" } 형식으로 JSON 데이터를 전달하세요.',
      };
    }
    try {
      const result = await this.naverMapService.getCoordinatesFromKeyword(address);
      return { 
        message: '장소 정보를 성공적으로 가져왔어요!', 
        data: result,
      };
    } catch (error) {
      return { 
        message: '오류가 발생했어요 😢', 
        error: error.message,
      };
    }
  }
}
