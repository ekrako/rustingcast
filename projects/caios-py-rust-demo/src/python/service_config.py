# type: ignore
from jdict import jdict
from caios.service.config_base import get_default_config_base

def get_configuration(service_name: str, mode: str) -> tuple[jdict, ...]:
    """
    Return a default configuration for development
    """
    return get_default_config_base()

