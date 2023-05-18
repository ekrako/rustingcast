# type: ignore
from caios.test import TestCase, main
from service import RustService
import time
from jdict import jdict

def fibonacci(n):
    return n if n < 2 else fibonacci(n-1) + fibonacci(n-2)


class TestFibsLocal(TestCase):
    def setUp(self):
        self._service = RustService()
        
    # def test_fibs_local(self):
    #     """ Local system test """
    #     resp = self._service.get_fib(40)
    #     self.assertEqual(102334155, resp.value)
    #     print(resp)
        
    # def test_fibs_local_python(self):
    #     actual_time = time.perf_counter()
    #     process_time = time.process_time()
    #     value = fibonacci(40)
    #     self.assertEqual(102334155, value)
    #     print(f'{value=} actual_time={time.perf_counter()-actual_time} process_time={time.process_time()-process_time}')

    def test_HW_local(self):
        resp=self._service.get_greetings(jdict(name='etzik'))
        print(resp)

if __name__ == "__main__":
    main()
