# 일반적인 컬렉션

러스트의 표준 라이브러리에는 *컬렉션*이라 불리는 여러 개의 매우 데이터 구조들이 포함되어 있다. 대부분의 다른 데이터 타입들은 하나의 특정한 값을 나타내지만, 컬렉션은 다수의 값을 담을 수 있다. 내장된 배열(build-in-array)와 튜플 타입과는 달리, 이 컬렉션들이 가리키고 있는 데이터들은 힙에 저장되는데, 이는 즉 데이터량이 컴파일 타임에 결정되지 않아도 프로그램이 실행될 때 늘어나거나 줄어들 수 있다는 의미이다. 

- 벡터(vector)는 여래 개의 값 서로 붙어 있게 저장
- 스트링(string)
- 해쉬맵(hash map) 어떤 값을 특정한 키와 연관지어 주도록 해준다. 이는 맵(map)이라 일컫는 좀 더 일반적인 데이터 구조의 특정한 구현 형태이다.