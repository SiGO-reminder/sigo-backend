# SiGO-backend: 시고 API 서버

## 📃 Description

시고 프로젝트에서 개발중인 API 통신 서버

### 개발 완료된 API endpoint 목록

- POST http://localhost:3000/naver-map/search?
  ```json
  { "address": "서울시청" }
  ```
- POST http://localhost:8080/api/v0/travel-time/transit
  ```json
  {
    "start_x": "127.02961696519651",
    "start_y": "37.58276551823443",
    "end_x": "127.0540211996305",
    "end_y": "37.58388311414532"
  }
  ```
- POST http://localhost:8080/api/v0/travel-time/driving
  ```json
  {
    "start_x": "127.02961696519651",
    "start_y": "37.58276551823443",
    "start_name": "안암오거리",
    "end_x": "127.0540211996305",
    "end_y": "37.58388311414532",
    "end_name": "해머스미스커피 서울시립대점"
  }
  ```
- POST http://localhost:8080/api/v0/travel-time/walking
  ```json
  {
    "start_x": "127.02961696519651",
    "start_y": "37.58276551823443",
    "start_name": "안암오거리",
    "end_x": "127.0540211996305",
    "end_y": "37.58388311414532",
    "end_name": "해머스미스커피 서울시립대점"
  }
  ```

## ✅ Project setup

개발 과정에서 서버를 두 곳으로 따로 분리해 개발을 진행하였으므로, 터미널을 두 개 열어 진행하도록 한다.
앞으로, 두 가지 서버는 각각 nestjs 서버 `/`, rust 서버 `/sigo-travel-time` 로 명명하도록 하겠다.

### 1. 프로젝트 clone

```bash
$ git clone https://github.com/SiGO-reminder/sigo-backend.git
$ cd sigo-backend
```

이 Repository를 로컬 환경에 clone한다.

### 2. 의존성 설치

- 2-1. nestjs 서버 의존성 설치

```bash
$ npm install
```

- 2-2. rust 서버 의존성 설치

```bash
$ cd sigo-travel-time
$ cargo build
```

### 3. .env 파일(환경변수 설정)

두 서버의 directory root에서 .env 파일을 생성하고, 필요한 환경변수를 추가해 준다.

```bash
$ touch .env
# 각 파일 내에 필요한 환경변수 입력
```

### 4. 서버 실행

두 서버를 실행시킨다.

- 4-1. nestjs 서버

```bash
$ npm run start
```

- 4-2. rust 서버

```bash
$ cargo run
```

### 5. API request & response 확인

- [위](#개발-완료된-api-endpoint-목록)에서 언급한 API 엔드포인트 목록 참고.
- curl, postman 등의 툴을 활용해서 테스트 해보세요 ~

## 🧪 Run tests (테스트 코드 추후 추가 예정)

```bash
# unit tests
$ npm run test

# e2e tests
$ npm run test:e2e

# test coverage
$ npm run test:cov
```

```bash
$ cargo test
```

## Deployment (TBC)

With Mau, you can deploy your application in just a few clicks, allowing you to focus on building features rather than managing infrastructure.

## Resources

Check out a few resources that may come in handy when working with NestJS:

- Visit the [NestJS Documentation](https://docs.nestjs.com) to learn more about the framework.
- For questions and support, please visit our [Discord channel](https://discord.gg/G7Qnnhy).
- To dive deeper and get more hands-on experience, check out our official video [courses](https://courses.nestjs.com/).
- Deploy your application to AWS with the help of [NestJS Mau](https://mau.nestjs.com) in just a few clicks.
- Visualize your application graph and interact with the NestJS application in real-time using [NestJS Devtools](https://devtools.nestjs.com).
- Need help with your project (part-time to full-time)? Check out our official [enterprise support](https://enterprise.nestjs.com).
- To stay in the loop and get updates, follow us on [X](https://x.com/nestframework) and [LinkedIn](https://linkedin.com/company/nestjs).
- Looking for a job, or have a job to offer? Check out our official [Jobs board](https://jobs.nestjs.com).

## Support

Nest is an MIT-licensed open source project. It can grow thanks to the sponsors and support by the amazing backers. If you'd like to join them, please [read more here](https://docs.nestjs.com/support).

## Stay in touch

- Author - [Kamil Myśliwiec](https://twitter.com/kammysliwiec)
- Website - [https://nestjs.com](https://nestjs.com/)
- Twitter - [@nestframework](https://twitter.com/nestframework)

## License

Nest is [MIT licensed](https://github.com/nestjs/nest/blob/master/LICENSE).
