import { NestFactory } from '@nestjs/core';
import { AppModule } from './app.module';

async function bootstrap() {
  const app = await NestFactory.create(AppModule);

  //전역 경로 프리픽스
  app.setGlobalPrefix('api/v0');

  await app.listen(process.env.PORT ?? 3000);
}
bootstrap();
