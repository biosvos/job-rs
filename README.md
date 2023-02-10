# 채용 공고 검색 툴

프로그래머스에서 채용 공고 탐색을 하는데 사이트를 너무 불편하게 만들어서 확김에 만들었다.

첫 러스트 프로젝트인 관계로 코드 퀄리티 양해바래요~.

## 사용 방법

해당 사이트의 공고를 가져와 보여준다.
`--refresh` 옵션이 모든 채용 공고를 끌어온다.
```bash
./target/debug/job-rs --refresh
```
> src/sourcer/programmers.rs 에 현재 최소 연봉 6000, 백엔드로 고정시켜놨다. 
> 파라미터로 넘겨야하는 거긴한데 거기까진 좀 귀찮...

필자는 검색까지는 만들기 귀찮아서 파일로 저장해서 sublime text 에서 검색한다. 
```bash
./target/debug/job-rs -o job.md
```
output 예시
```
# ~~무슨 무슨 회사~~
- [~~채용공고 제목~~](~~채용 공고 접속 url~~)
  - Database, Programming Language/Python, Web/Node.js, Programming Language/Java
  - 조건1
  - 조건2
  - 조건...
```
> 보면 현재 태깅을 채용 공고 제목과 조건을 가지고 자동으로 만들어낸다.
> 
> 해당 태그로 검색하면 편하다.