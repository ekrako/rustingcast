import time
from jdict import jdict
from fibrs import fibonacci, helloworld


class RustService:
    """Demonstrates Fibonacci calculator in Rust using CAIOS"""
        
    # def get_fib(self, fib_id: str) ->jdict:
    #     """ Creates a new greeting
            
    #         :param str fib_id: which Fibonacci number to generate
    #         :return: Fibonacci number value and timing information

    #     """
    #     actual_time = time.perf_counter()
    #     process_time = time.process_time()
    #     value = fibonacci(int(fib_id))
    #     return jdict(
    #         value=value,
    #         actual_time=time.perf_counter() - actual_time,
    #         process_time=time.process_time() - process_time
    #     )
        
    def get_greetings(self,person:jdict) ->None:
        print('Py function started')
        resp=helloworld(person.name)
        return resp
