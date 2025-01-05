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
  async getGeocode(address: string) {
    // 환경 변수에서 네이버 API 키를 불러옴
    const clientId = this.configService.get<string>('NAVER_CLIENT_ID');
    const clientSecret = this.configService.get<string>('NAVER_CLIENT_SECRET');

    // 네이버 지도 API의 Geocode URL
    const url = `https://naveropenapi.apigw.ntruss.com/map-geocode/v2/geocode`;

    // API 요청 시 필요한 헤더
    const headers = {
      'X-NCP-APIGW-API-KEY-ID': clientId, // 클라이언트 ID
      'X-NCP-APIGW-API-KEY': clientSecret, // 클라이언트 Secret
    };

    // 네이버 지도 API에 HTTP GET 요청을 보냄
    const response$ = this.httpService.get(url, {
      headers,
      params: { query: address }, // 요청 파라미터에 주소(query)를 포함
    });

    // RxJS Observable을 Promise로 변환하여 비동기 결과를 기다림
    const response = await lastValueFrom(response$);
    
    // API 응답 데이터 반환
    return response.data;
  }
}

