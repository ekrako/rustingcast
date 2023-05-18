# type: ignore
from caios.test import TestCase, main, test_service
import warnings
import requests


class TestFibsRemote(TestCase):
    def setUp(self):
        self._url = f'{test_service.outputs.http_api}/fibs/'

        
    def test_fibs_remote(self):
        """ Remote test """
        # warnings.filterwarnings(action="ignore", message="unclosed", category=ResourceWarning)
        resp=requests.get(f'{self._url}/40')
        self.assertEqual(200, resp.status_code)
        data = resp.json()
        self.assertEqual(102334155, data.value)
        print(resp.status_code, data)


if __name__ == "__main__":
    main()
