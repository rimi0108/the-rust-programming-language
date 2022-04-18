## 소유권이란
러스트의 핵심 기능은 바로 소유권(ownership)이다. 

모든 프로그램은 실행하는 동안 컴퓨터의 메모리를 사용하는 방법을 관리해야 한다. 몇몇 언어들은 프로그램이 실행될 때 더이상 사용하지 않는 메모리를 끊임없이 찾는 **가비지 콜렉션**을 갖고 있다. 다른 언어들에서는 프로그래머가 직접 명시적으로 메모리를 할당하고 해제해야 한다. 러스트는 제 3의 접근법을 이용한다. 메모리는 컴파일 타임에 컴파일러가 체크할 규칙들로 구성된 소유권 시스템을 통해 관리된다. 소유권 기능들의 어떤 것도 런타임 비용이 발생하지 않는다. 

소유권을 이해했을 때, 러스트를 유니크하게 만드는 기능들을 이해하기 위한 견고한 기초를 가지게 될 것이다.

### 스택과 힙 (Stack & Heap)
많은 프로그래밍 언어들 안에서 우리는 그렇게 자주 스택과 힙 생각을 할 필요가 없다. 그렇지만 Rust와 같은 시스템 프로그래밍 언어에서는, 값이 스택에 있는지 힙에 있는지의 여부가 언어의 동작 방식과 우리의 결단에 큰 영향을 준다.

스택과 힙 둘 다 코드상에서 런타임에 사용할 수 있는 메모리의 부분이다. 이들은 각기 다른 방식으로 구조화 되어 있다. 스택(Stack)은 값을 받아들인 순서대로 값을 저장하고 반대 방향으로 값을 지운다. 이것을 last in first out 이라고 한다. (쌓여있는 접시) 데이터를 추가하는 것을 스택에 푸시하기 (pushing on the stack)이라고 부르고, 데이터를 제거하는 것을 스택을 팝하기 (popping off the stack)이라고 부른다.

#### 스택

![image](https://user-images.githubusercontent.com/73830753/163744055-32554b8a-225e-4245-a841-ef95ad134c2d.png)
https://pediaa.com/what-is-the-difference-between-stack-and-heap/

스택은 데이터가 접근하는 방식 덕에 빠르다. 이 방식은 새로운 데이터를 넣어두기 위한 공간 혹은 데이터를 가져올 공간을 검색할 필요가 전혀 없는데, 바로 그 공간이 항상 스택의 꼭대기(top)이기 때문이다. 스택을 빠르게 해주는 또 다른 특성은 스택에 담긴 모든 데이터가 결정되어 있는 고정된 크기를 갖고 있어야 한다는 점이다.

#### 힙

![image](https://user-images.githubusercontent.com/73830753/163744121-d318674f-08a7-421b-a54d-c1cc82f14087.png)
https://pediaa.com/what-is-the-difference-between-stack-and-heap/

컴파일 타임에 *크기가 결정되어 있지 않거나* *크기가 변경될 수 있는 데이터*를 위해서는, 힙에 데이터를 저장할 수 있다. 힙은 스택보다 조금 더 복잡하다. 데이터를 힙에 넣을 때, 먼저 저장할 공간이 있는지 물어본다. 그러면 운영체제는 충분히 커다란 힙 안의빈 어떤 지점을 찾아 이 곳을 사용중이라고 표시하고, 해당 지점의 포인터를 우리에게 돌려준다. 이 절차를 힙 공간 할당하기(allocating on the heap)이라고 부르고, 종종 그냥 "할당(allocating)" 이라고 줄어 부른다. 스택에 포인터를 푸싱하는 것은 할당에 해당되지 않는다. 포인터는 결정되어 있는 고정된 크기의 값이므로, 우리는 스택에 포인터를 저장할 수 있지만, 실제 데이터를 사용하고자 할 때는 포인터를 따라가야 한다.

힙에 저장된 데이터에 접근하는 것은 스택에 저장된 데이터에 접근하는 것보다 느린데, 그 이유는 포인터가 가리킨 곳을 따라가야 하기 때문이다. 현대 프로세서들은 메모리 내부를 덜 탐색할 때 더 빨라진다. 유사한 예로, 여러 테이블로부터 주문을 받는 레스토랑의 웨이터를 생각해보면 다음 테이블로 움직이기 전에 지금 테이블에서 모든 주문을 다 받는 것이 효율적이다. A 테이블에서 하나 주문 받고, 다시 B 테이블로 가서 다시 주문을 받고, 다시 A로, 다시 B로 가며 하나씩 주문을 받으면 훨씬 느려질 것이다. 이와 마찬가지로, 프로세서는 (힙에 있는 데이터와 같이) 멀리 떨어져 있는 데이터 보다는 (스택에 있는 것과 같이) 붙어있는 데이터들에 대한 작업을 하면 더 빨라진다. 힙으로부터 큰 공간을 할당받는 것 또한 시간이 걸릴 수 있다. 

코드의 어떤 부분이 힙의 어떤 데이터를 사용하는지 추적하는 것, 힙의 중복된 데이터의 양을 최소화하는 것, 그리고 힙 내에 사용되지 않는 데이터를 제거하여 공간이 모자라지 않게 하는 것은 모두 소유권과 관계된 문제들이다. 

### 소유권 규칙

1. 러스트의 각각의 값은 해당값의 오너(owner)라고 불리우는 변수를 갖고 있다.
2. 한번에 딱 하나의 오너만 존재할 수 있다.
3. 오너가 스코프 밖으로 벗어나는 때, 값은 버려진다(dropped).

### 메모리와 할당
스트링 리터럴의 경우, 우리는 내용물을 컴파일 타임에 알 수 있으므로 텍스트가 최종 실행파일에 직접 하드코딩 되었고, 이렇게 하면 스트링 리터럴이 빠르고 효율적이다. 그러나 이는 문자열이 변경되지 않는 것을 전재로 하는 특성이다. 불행하게도, 우리는 컴파일 타임에 크기를 알 수 없는 경우 및 실행 중 크기가 변할 수도 있는 경우의 텍스트 조각을 바이너리 파일에 집어넣을 수 없다.

String 타입은 변경 가능하고 커질 수 있는 텍스트를 지원하기 위해 만들어졌고, 우리는 힙에서 컴파일 타임에는 알 수 없는 어느 정도 크기의 메모리 공간을 할당받아 내용물을 저장할 필요가 있다.

1. 런타임에 운영체제로부터 메모리가 요청되어야 한다.
2. String의 사용이 끝났을 때 운영체제에게 메모리를 반납할 방법이 필요하다.

첫번째는 우리가 직접 수행하는 것이다. 우리가 `String::from` 을 호출하면, 구현부분에서 필요한 만큼의 메모리를 요청한다. 이것은 프로그래밍 언어들 사이에서 매우 일반적이다.

하지만 두번째는 다르다. 가비지 콜렉터(GC)를 갖고 있는 언어들의 경우, GC가 더이상 사용하지 않는 메모리 조각을 계속해서 찾고 지워주며, 우리는 프로그래머로서 이와 관련된 생각을 안해도 된다. GC가 없을 경우, 할당받은 메모리가 더 필요없는 시점을 알아서 명시적으로 이를 반납하는 코드를 호출하는 것은 프로그래머의 몫이다. 이를 올바르게 하는 것은 역사적으로 어려운 문제로 취급 받았다. 프로그래머가 이것을 잊어버린다면 메모리를 낭비하는 것이다. 너무 빨리 반납해버리면 유효하지 않는 변수를 갖게 된다. 만일 반납을 두 번 하면 이것도 버그다. 우리는 딱 한번의 `allocate`와 한번의 `free` 쌍을 사용해야 한다.

러스트는 다른 방식으로 이 문제를 다룬다. 메모리는 변수가 소속되어 있는 스코프 밖으로 벗어나는 순간 자동으로 반납된다.

```rust
{
    let s = String::from("hello"); // s는 여기서부터 유효합니다

    // s를 가지고 뭔가 합니다
}                                  // 이 스코프는 끝났고, s는 더 이상 
                                   // 유효하지 않습니다
```
String이 요구한 메모리를 운영체제에게 반납하는 자연스러운 지점이 있다. s가 스코프 밖을 벗어날 때이다. 변수가 스코프 밖으로 벗어나면, 러스트는 우리를 위해 특별한 함수를 호출한다. 이 함수를 `drop`이라고 불고, String의 개발자가 메모리를 반환하도록 하는 코드를 집어넣을 수 있다. 러스트는 `}` 괄호가 닫힐 때 자동적으로 `drop`을 호출한다.

#### 변수와 데이터가 상호작용하는 방법: 이동(move)

여러 개의 변수들은 러스트에서 서로 다른 방식으로 같은 데이터에 대해 상호 작용을 할 수 있다. 

String은 밑 그림과 같이 생겼다. String은 그림의 왼쪽과 같이 세 개의 부분으로 이루어져 있다. 문자열의 내용물을 담고 있는 메모리의 포인터(ptr), 길이(len), 그리고 용량(capacity)이다. 이 데이터의 그룹은 스택에 저장된다. 내용물을 담은 오른쪽의 것은 힙 메모리에 있다.

![image](https://user-images.githubusercontent.com/73830753/163746358-a5e509cb-c52e-4656-8e03-1b7fe500d721.png)

[Figure 4-3: s1 변수에 "hello" 값이 저장된 String의 메모리 구조]

길이 값은 바이트 단위로 String의 내용물이 얼마나 많은 메모리를 현재 사용하고 있는지를 말한다. 용량값은 바이트 단위로 String이 운영체제로부터 얼마나 많은 메모리를 할당 받았는지를 말한다. 길이와 용량의 차이는 중요하지만 이번 내용에서는 아니니 무시하셔도 좋다.

s2에 s1을 대입하면 String 데이터가 복사되는데, 이는 스택에 있는 포인터, 길이값, 그리고 용량값이 복사된다는 의미이다. 포인터가 가르키고 있는 힙 메모리 상의 데이터는 복사되지 않는다. 달리 말하면, 메모리 내의 데이터구조는 밑의 그림과 같다.

![image](https://user-images.githubusercontent.com/73830753/163746836-d81e8a27-bb50-4165-bca6-c329aff74d98.png)

[Figure 4-4: s1의 포인터, 길이값, 용량값이 복사된 s2 변수의 메모리 구조]

![image](https://user-images.githubusercontent.com/73830753/163746934-5bdf478c-feb0-45e8-a012-b52498b99f28.png)

[Figure 4-5: 러스트가 힙 데이터까지 복사하게 될 경우 's2 = s1'가 만들 또다른 가능성]

위 그림은 러스트가 힙 메모리 상의 데이터까지도 복사한다면 벌어질 일이다. 만일 러스트가 이렇게 동작한다면, 힙 안의 데이터가 클 경우 s2 = s1 연산은 런타임 상에서 매우 느려질 가능성이 있다.

앞 서 우리는 변수가 스코프 밖으로 벗어날 때, 러스트는 자동적으로 `drop` 함수를 호출하여 해당 변수가 사용하는 힙 메모리를 제거한다고 했다. 하지만 `Figure 4-4` 에서는 두 데이터 포인터가 모두 같은 곳을 가리키고 있는 것을 확인할 수 있다. 이것은 문제가 된다. s2와 s2이 스코프 밖으로 벗어나게 되면, 둘 다 같은 메모리를 해제하려 할 것이다. 이는 **두번 해제 (double free)** 오류라고 알려져 잇으며 이전에 언급한 메모리 안정성 버그들 중 하나이다. 메모리를 두 번 해제하는 것은 메모리 손상(memory corruption)의 원인이 되는데, 이는 보안 취약성 문제를 일으킬 가능성이 있다.

메모리 안정성을 보장하기 위해서, 러스트에서는 이런 경우 어떤 일이 일어나는 지 한 가지 디테일이 더 있다. 할당된 메모리를 복사하는 것을 시도하는 대신, 러스트에서는 s1이 더이상 유효하지 않다고 간주하고, 러스트는 s1이 스코프 밖을 벗어났을 때 아무것도 해제하지 않는다.

s1을 s2가 만들어진 후에 사용하려고 하면 에러가 발생한다.

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```
```
error[E0382]: use of moved value: `s1`
 --> src/main.rs:4:27
  |
3 |     let s2 = s1;
  |         -- value moved here
4 |     println!("{}, world!", s1);
  |                            ^^ value used here after move
  |
  = note: move occurs because `s1` has type `std::string::String`,
which does not implement the `Copy` trait
```
에러가 발생하는 이유는 러스트가 유효하지 않는 참조를 사용하는 것을 막기 때문이다.

만일 다른 언어로 프로그래밍 하는 동안 "얕은 복사(shallow copy)"와 "깊은 복사(deep copy)"라는 용어를 들어봤다면, 데이터의 복사 없이 포인터와 길이값 및 용량값만 복사한다는 개념이 