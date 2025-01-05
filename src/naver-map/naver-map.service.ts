import { Injectable } from '@nestjs/common';  //NestJS 의존성 주입(DI)
import { ConfigService } from '@nestjs/config';  //.env 파일에서 환경변수 읽기
import { HttpService } from '@nestjs/axios';  //HTTP 요청을 보내기 위한 NestJS 모듈
import { lastValueFrom } from 'rxjs'; // RxJS의 Observable을 Promise로 변환(단일 비동기 작업이므로 / async await 쓰려고)

@Injectable() // 이 클래스가 서비스임을 나타냄
export class NaverMapService {
  constructor(
    private readonly configService: ConfigService, // 환경 변수 접근
    private readonly httpService: HttpService, // HTTP 요청을 위한 HttpService 주입
  ) {}

  // 네이버 지도 API에서 주소의 좌표를 가져오는 함수
  async getCoordinatesFromKeyword(keyword: string): Promise<any> {
    // 환경 변수에서 네이버 API 키를 불러옴
    const clientId = this.configService.get<string>('NAVER_CLIENT_SEARCHID');
    const clientSecret = this.configService.get<string>('NAVER_CLIENT_SEARCHSECRET');

    // 네이버 지도 API의 장소 검색 URL
    const url = `https://openapi.naver.com/v1/search/local.json`;

    // HTTP 요청 보내기
    const response$ = this.httpService.get(url, {
        headers: {
          'X-Naver-Client-Id': clientId,
          'X-Naver-Client-Secret': clientSecret,
        },
        params: {
            query: keyword, // 검색 키워드
            display: 4, // 결과 최대 4개만 반환
          },
      });

      const response = await lastValueFrom(response$); // Observable을 Promise로 변환
      const items = response.data.items; // 검색 결과 배열
  
      if (items.length === 0) {
        throw new Error('해당 키워드에 대한 검색 결과가 없습니다.');
      }
  
    // 상위 4개 결과를 필터링하여 필요한 정보만 반환
    const topResults = items.slice(0, 4).map((item: any) => ({
        name: item.title.replace(/<[^>]*>/g, ''), // HTML 태그 제거
        address: item.address,
        category: item.category,
        x: item.mapx,
        y: item.mapy,
      }));
  
      return topResults;
    }
}

