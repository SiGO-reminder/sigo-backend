import { Module } from '@nestjs/common';
import { NaverMapService } from './naver-map.service';
import { NaverMapController } from './naver-map.controller';
import { HttpModule } from '@nestjs/axios';

@Module({
  imports: [HttpModule],
  providers: [NaverMapService],
  controllers: [NaverMapController],
  exports: [NaverMapService],
})
export class NaverMapModule {}
