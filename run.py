import os

DIR = os.path.dirname(os.path.abspath(__file__))
# os.environ['PYTHONPYCACHEPREFIX'] = f'{DIR}/__pycache__'

if __name__ == "__main__":
    import uvicorn
    uvicorn.run("api.main:app", host="0.0.0.0", port=8000, reload=True)