import openai
import os
from tqdm import tqdm

# Load your OpenAI API key from an environment variable
openai.api_key = os.getenv('OPENAI_API_KEY')

if openai.api_key is None:
    raise ValueError("No OpenAI API key found. Set the OPENAI_API_KEY environment variable.")

def generate_animal_names(prompt, n=50):
    try:
        # Generate a batch of animal names
        messages = [{"role": "system", "content": "You are a helpful assistant."},
                    {"role": "user", "content": prompt}]
        response = openai.ChatCompletion.create(
            model="gpt-3.5-turbo",
            messages=messages
        )
        # The response will be a list of messages, we want the last one which contains the assistant's response
        names = response['choices'][0]['message']['content']
        # Extract the names from the assistant's response, assuming they're separated by line breaks
        names = names.strip().split('\n')
        if len(names) > n:
            names = names[:n]
        return names
    except Exception as e:
        print(f"An error occurred: {e}")
        return []

def main():
    total_names = 1000
    batch_size = 50
    prompt = "Give me 50 fun and unique animal names"

    # Open a file to save the animal names
    with open('animal_names.txt', 'w') as file:
        for _ in tqdm(range(0, total_names, batch_size)):
            # Generate a batch of animal names
            animal_names = generate_animal_names(prompt, batch_size)
            # Write the batch of animal names to the file
            if animal_names:
                for name in animal_names:
                    file.write(name + '\n')
                print(f"Saved {len(animal_names)} animal names to file.")
            else:
                print("No names were generated in this batch.")

if __name__ == "__main__":
    main()
