import numpy as np
import matplotlib.pyplot as plt
plt.rcParams.update({'font.size': 24})  # font size

def make_plot(title, data, path):

    plt.figure(figsize=(20.3, 13))

    plt.title(title)
    plt.ylabel('Tempo (em Milissegundos)')
    plt.xlabel('Tamanho da Chave (em bits)')

    plt.xticks(np.arange(min(data["xaxis"]), max(data["xaxis"]), 2))
    for label in data.keys():
        if label == "xaxis":
            continue
        plt.plot(data["xaxis"], data[label], label=label)
        


    plt.legend(loc = 'upper left')

    plt.savefig(title + ".png")

def make_plot_only_encryption(title, data, path):

    plt.figure(figsize=(20.3, 13))

    plt.title(title)
    plt.ylabel('Tempo (em Milissegundos)')
    plt.xlabel('Tamanho da Chave (em bits)')

    plt.xticks(np.arange(min(data["xaxis"]), max(data["xaxis"])+1, 64))
    plt.plot(data["xaxis"], data["encryption"], label="encryption")
        


    plt.legend(loc = 'upper left')

    plt.savefig(title + ".png")

def averageOfList(list):
    return sum(list)/len(list)

if __name__ == "__main__":

    xaxis = []
    breaking_line = []
    decryption_line = []
    encryption_line = []
    generation_line = []
    
    for i in range(32,75,2):
        xaxis.append(i)

        # breaking
        breaking_out = open(f'./output/breaking/{i}', "r")
        breaking = breaking_out.read().split('\n')
        breaking.pop()
        breaking = list(map(int, breaking))

        breaking_line.append( averageOfList(breaking) )

        # decryption
        decryption_out = open(f'./output/decryption/{i}', "r")
        decryption = decryption_out.read().split('\n')
        decryption.pop()
        decryption = list(map(int, decryption))

        decryption_line.append( averageOfList(decryption) )

        # encryption
        encryption_out = open(f'./output/encryption/{i}', "r")
        encryption = encryption_out.read().split('\n')
        encryption.pop()
        encryption = list(map(int, encryption))

        encryption_line.append( averageOfList(encryption) )

        # generation
        generation_out = open(f'./output/generation/{i}', "r")
        generation = generation_out.read().split('\n')
        generation.pop()
        generation = list(map(int, generation))

        generation_line.append( averageOfList(generation) )

    generated_data = {
        "xaxis": xaxis,
        "breaking": breaking_line,
        "decryption": decryption_line,
        "encryption": encryption_line,
        "generation": generation_line,
    }

    breaking_data = {
        "xaxis": xaxis,
        "breaking": breaking_line,
    }

    decryption_data = {
        "xaxis": xaxis,
        "decryption": decryption_line,
    }

    encryption_data = {
        "xaxis": xaxis,
        "encryption": encryption_line,
    }

    generation_data = {
        "xaxis": xaxis,
        "generation": generation_line,
    }

    make_plot("Dados Gerais Obtidos", generated_data, "./output")
    make_plot("Dados de Breaking", breaking_data, "./output")
    make_plot("Dados de decryption", decryption_data, "./output")
    make_plot("Dados de generation", generation_data, "./output")


    # ONLY ENCRYPTION
    only_encryption_line = []
    only_encryption_xaxis = []
    for i in range(64, 1025, 64):
        only_encryption_out = open(f'./output/only_encryption/{i}', "r")
        only_encryption = only_encryption_out.read().split('\n')
        only_encryption.pop()
        only_encryption = list(map(int, only_encryption))
        only_encryption_line.append( averageOfList(only_encryption) )

        only_encryption_xaxis.append(i)

    only_encryption_data = {
        "xaxis": only_encryption_xaxis,
        "encryption": only_encryption_line,
    }

    print(only_encryption_data)
    make_plot_only_encryption("Dados de Encryption", only_encryption_data, "./output")
